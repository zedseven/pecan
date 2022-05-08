// Uses
use std::borrow::Cow;

use chrono::Utc;
use diesel::{
	dsl::exists,
	insert_into,
	result::OptionalExtension,
	select,
	sql_query,
	sql_types::{Integer, Nullable, Text},
	update,
	BelongingToDsl,
	Connection,
	ExpressionMethods,
	GroupedBy,
	QueryDsl,
	RunQueryDsl,
};
use rocket::{get, post, routes, Route};
use rocket_contrib::{
	json,
	json::{Json, JsonValue},
};

use super::Routable;
use crate::{
	db::{
		models::*,
		schema,
		util::{data_value_exists, fetch_new_rowid_on},
		DbConn,
	},
	error::{Context, Error, UserError},
	util::{gen_new_component_id, gen_new_device_id},
};

/// The route for this section.
pub(super) struct DevicesApi;
impl Routable for DevicesApi {
	const PATH: &'static str = "/devices";
	const ROUTES: &'static dyn Fn() -> Vec<Route> = &|| {
		routes![
			get_definitions,
			search_devices,
			get_device,
			checkout_device,
			create_device,
			update_device,
			get_data_value_exists
		]
	};
}

// Type Definitions
#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NewDeviceInfo {
	location_id: i32,
	column_data: Vec<SubmittedColumnData>,
	components: Vec<NewDeviceComponent>,
}
#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdatedDeviceInfo {
	location_id: i32,
	column_data: Vec<SubmittedColumnData>,
	components: Vec<UpdatedDeviceComponent>,
}
#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NewDeviceComponent {
	component_type: String,
}
#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdatedDeviceComponent {
	component_id: Option<String>,
	component_type: String,
}
#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SubmittedSearchQuery {
	device_id: String,
	location_id: Option<i32>,
	column_data: Vec<SubmittedColumnData>,
}
#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SubmittedColumnData {
	column_definition_id: i32,
	data_value: String,
}
#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CheckoutInfo {
	device_id: String,
	location_id: i32,
}
#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ValueExistsQuery {
	device_id: Option<String>,
	value: String,
}

/// Fetches the column definitions and locations.
#[get("/definitions")]
pub fn get_definitions(mut conn: DbConn) -> Result<JsonValue, Error> {
	use schema::{column_definitions::dsl::*, column_possible_values::dsl::*, locations::dsl::*};

	let column_definition_results = column_definitions
		.load::<ColumnDefinition>(&mut conn.0)
		.with_context("unable to load the column definitions")?;

	let possible_values_results = ColumnPossibleValue::belonging_to(&column_definition_results)
		.order_by(value)
		.load::<ColumnPossibleValue>(&mut conn.0)
		.with_context("unable to load the column possible values")?
		.grouped_by(&column_definition_results);

	let column_results = column_definition_results
		.into_iter()
		.zip(possible_values_results)
		.collect::<Vec<_>>();

	let location_results = locations
		.load::<LocationDefinition>(&mut conn.0)
		.with_context("unable to load the locations")?;

	Ok(json!({ "columnDefinitions": column_results, "locations": location_results }))
}

/// Fetches a device by ID.
#[get("/get/<device>")]
pub fn get_device(mut conn: DbConn, device: String) -> Result<JsonValue, Error> {
	// Uses
	use schema::{device_key_info::dsl::*, locations::dsl::*};

	// Load from the database
	let device_key_info_results = device_key_info
		.filter(device_id.eq(device.as_str()))
		.inner_join(locations)
		.select(DEVICE_INFO)
		.get_result::<DeviceInfo>(&mut conn.0)
		.optional()
		.with_context("unable to load device info")?;
	if device_key_info_results.is_none() {
		return Err(UserError::NotFound("Invalid device ID.").into());
	}
	let device_key_info_results = device_key_info_results.unwrap();

	let device_data_results = DeviceData::belonging_to(&device_key_info_results)
		.get_results::<DeviceData>(&mut conn.0)
		.with_context("unable to load the device data")?;

	let device_component_results = DeviceComponent::belonging_to(&device_key_info_results)
		.get_results::<DeviceComponent>(&mut conn.0)
		.with_context("unable to load the device components")?;

	// Return the results
	// This odd return format is to match how the data is returned in search
	// results.
	Ok(json!({
		"deviceResults": (device_key_info_results, device_data_results),
		"deviceComponents": device_component_results
	}))
}

/// Fetches the most recently-updated `count` entries.
#[post("/search", data = "<search_query>")]
pub fn search_devices(
	mut conn: DbConn,
	search_query: Json<SubmittedSearchQuery>,
) -> Result<JsonValue, Error> {
	// Check if any column values were specified for searching
	let search_column_data_is_present = search_query
		.column_data
		.iter()
		.any(|column| !column.data_value.is_empty());

	// Search the device key info
	// This whole thing is *extremely* ugly. This is because Diesel doesn't support
	// boxed sub-queries, because the boxing operation can't know that it will only
	// be used in a sub-query where the referenced parent column is valid.
	// The boxing operation is required because we dynamically add filter conditions
	// to the query.
	let mut search_sql = String::from(
		"SELECT
		    'dki'.'id',
		    'dki'.'device_id',
		    'dki'.'location_id',
		    'l'.'name' AS 'location',
		    'dki'.'last_updated'
		FROM 'device_key_info' AS 'dki'
		INNER JOIN 'locations' AS 'l' ON 'l'.'id' = 'dki'.'location_id'
		WHERE
		    'dki'.'device_id' LIKE ?
		    AND (? IS NULL OR 'dki'.'location_id' = ?)\n",
	);
	let mut bind_params = Vec::new();
	if search_column_data_is_present {
		search_sql.push_str(
			"AND ? = (SELECT
               COUNT('dd'.'id')
           FROM 'device_data' AS 'dd'
           WHERE 'dd'.'device_key_info_id' = 'dki'.'id' AND (",
		);
		let mut first_entry = true;
		for column_query in &search_query.column_data {
			if column_query.data_value.is_empty() {
				continue;
			}
			if !first_entry {
				search_sql.push_str(" OR ");
			}
			search_sql.push_str("('dd'.'column_definition_id' = ? AND 'dd'.'data_value' LIKE ?)");
			bind_params.push((
				column_query.column_definition_id,
				format!("%{}%", column_query.data_value.as_str()),
			));
			first_entry = false;
		}
		search_sql.push_str("))\n");
	}
	search_sql.push_str("ORDER BY 'dki'.'last_updated' DESC");
	// dbg!(&search_sql);
	let mut device_key_info_query = sql_query(search_sql)
		.into_boxed()
		.bind::<Text, _>(format!("%{}%", search_query.device_id.as_str()))
		.bind::<Nullable<Integer>, _>(search_query.location_id)
		.bind::<Nullable<Integer>, _>(search_query.location_id);
	if search_column_data_is_present {
		device_key_info_query = device_key_info_query.bind::<Integer, _>(bind_params.len() as i32);
	}
	for (bind_column_definition_id, bind_data_value_search) in bind_params {
		device_key_info_query = device_key_info_query
			.bind::<Integer, _>(bind_column_definition_id)
			.bind::<Text, _>(bind_data_value_search);
	}
	let device_key_info_results = device_key_info_query
		.load::<DeviceInfoByName>(&mut conn.0)
		.with_context("unable to load device info")?
		.drain(..)
		.map(Into::into)
		.collect::<Vec<DeviceInfo>>();
	// dbg!(&device_key_info_results);

	// Collect the device data
	let device_data_results = DeviceData::belonging_to(&device_key_info_results)
		.load::<DeviceData>(&mut conn.0)
		.with_context("unable to load the device data")?
		.grouped_by(&device_key_info_results);

	// Bring it together
	let device_results = device_key_info_results
		.into_iter()
		.zip(device_data_results)
		.collect::<Vec<_>>();

	// Return the results
	Ok(json!({ "deviceResults": device_results }))
}

#[post("/checkout", data = "<checkout_info>")]
pub fn checkout_device(
	mut conn: DbConn,
	checkout_info: Json<CheckoutInfo>,
) -> Result<JsonValue, Error> {
	// Uses
	use schema::{device_key_info::dsl::*, locations::dsl::*};

	// Verify the new location
	let location_name = locations
		.filter(schema::locations::dsl::id.eq(checkout_info.location_id))
		.select(schema::locations::dsl::name)
		.get_result::<String>(&mut conn.0)
		.optional()
		.with_context("unable to query the database for location existence")?;
	if location_name.is_none() {
		return Err(UserError::NotFound("Invalid location.").into());
	}
	let location_name = location_name.unwrap();

	// Fetch the device's internal ID
	let internal_id = device_key_info
		.filter(device_id.eq(checkout_info.device_id.as_str()))
		.select(schema::device_key_info::dsl::id)
		.get_result::<i32>(&mut conn.0)
		.with_context("unable to get the internal ID associated with the provided device ID")?;

	// Update the device entry
	update(device_key_info.filter(schema::device_key_info::dsl::id.eq(internal_id)))
		.set((
			location_id.eq(checkout_info.location_id),
			last_updated.eq(Utc::now().naive_utc()),
		))
		.execute(&mut conn.0)
		.with_context("unable to update device_key_info")?;

	// Return the results
	Ok(json!({
		"deviceId": checkout_info.device_id.clone(),
		"locationId": checkout_info.location_id,
		"locationName": location_name
	}))
}

/// Adds a new device to the database.
#[post("/create", data = "<device_info>")]
pub fn create_device(
	mut conn: DbConn,
	device_info: Json<NewDeviceInfo>,
) -> Result<JsonValue, Error> {
	// Uses
	use schema::{
		device_components::dsl::*,
		device_data::dsl::*,
		device_key_info::dsl::*,
		locations::dsl::*,
	};

	// Verify the new location
	if !select(exists(
		locations.filter(schema::locations::dsl::id.eq(device_info.location_id)),
	))
	.get_result::<bool>(&mut conn.0)
	.with_context("unable to query the database for location existence")?
	{
		return Err(UserError::NotFound("Invalid location.").into());
	}

	// Generate a new device ID
	let new_device_id =
		gen_new_device_id(&mut conn.0).with_context("unable to generate a new device ID")?;

	// Begin the transaction
	conn.0
		.transaction::<_, Error, _>(|conn| {
			// Insert the new device entry
			insert_into(device_key_info)
				.values(DeviceKeyInfoNew {
					device_id: Cow::from(new_device_id.as_str()),
					location_id: device_info.location_id,
					last_updated: Utc::now().naive_utc(),
				})
				.execute(conn)
				.with_context("unable to insert into device_key_info")?;

			// Fetch the new device's internal ID
			let new_device_key_info_id = fetch_new_rowid_on(conn)
				.with_context("unable to get the new device_key_info id")?;

			// Insert the device column data
			insert_into(device_data)
				.values(
					device_info
						.column_data
						.iter()
						.map(|data| DeviceDataNew {
							device_key_info_id: new_device_key_info_id,
							column_definition_id: data.column_definition_id,
							data_value: Cow::from(data.data_value.as_str()),
						})
						.collect::<Vec<_>>(),
				)
				.execute(conn)
				.with_context("unable to insert into device_data")?;

			// Insert all of the components
			for component in &device_info.components {
				// This could definitely be made more performant, but in this case it's not
				// really necessary and this is convenient
				let new_component_id = gen_new_component_id(conn, new_device_key_info_id)
					.with_context("unable to generate a new component ID")?;

				// Insert the component
				insert_into(device_components)
					.values(DeviceComponentNew {
						device_key_info_id: new_device_key_info_id,
						component_id: Cow::from(new_component_id.as_str()),
						component_type: Cow::from(component.component_type.as_str()),
					})
					.execute(conn)
					.with_context("unable to insert into device_components")?;
			}

			Ok(())
		})
		.with_context("unable to create the new device entry")?;

	// Return the results
	Ok(json!({ "deviceId": new_device_id }))
}

/// Updates a device's data.
///
/// TODO: Combine this with `create_device`.
#[post("/update/<device>", data = "<device_info>")]
pub fn update_device(
	mut conn: DbConn,
	device: String,
	device_info: Json<UpdatedDeviceInfo>,
) -> Result<JsonValue, Error> {
	// Uses
	use schema::{
		device_components::dsl::*,
		device_data::dsl::*,
		device_key_info::dsl::*,
		locations::dsl::*,
	};

	// Verify the new location
	if !select(exists(
		locations.filter(schema::locations::dsl::id.eq(device_info.location_id)),
	))
	.get_result::<bool>(&mut conn.0)
	.with_context("unable to query the database for location existence")?
	{
		return Err(UserError::NotFound("Invalid location.").into());
	}

	// Technically there should be verification that duplicate values aren't being
	// submitted here, but it's already enforced by the frontend and it's not worth
	// the many additional queries right now
	// The same goes for not-null values

	// Fetch the device's internal ID
	let internal_id = device_key_info
		.filter(device_id.eq(device.as_str()))
		.select(schema::device_key_info::dsl::id)
		.get_result::<i32>(&mut conn.0)
		.with_context("unable to get the internal ID associated with the provided device ID")?;

	// Begin the transaction
	conn.0
		.transaction::<_, Error, _>(|conn| {
			// Update the device entry
			update(device_key_info.filter(schema::device_key_info::dsl::id.eq(internal_id)))
				.set((
					location_id.eq(device_info.location_id),
					last_updated.eq(Utc::now().naive_utc()),
				))
				.execute(conn)
				.with_context("unable to update device_key_info")?;

			// Update the device column data
			for column in &device_info.column_data {
				update(
					device_data
						.filter(schema::device_data::dsl::device_key_info_id.eq(internal_id))
						.filter(column_definition_id.eq(column.column_definition_id)),
				)
				.set(data_value.eq(column.data_value.as_str()))
				.execute(conn)
				.with_context("unable to update device_data")?;
			}

			// Update the device components
			for component in &device_info.components {
				if let Some(existing_id) = &component.component_id {
					update(
						device_components
							.filter(
								schema::device_components::dsl::device_key_info_id.eq(internal_id),
							)
							.filter(component_id.eq(existing_id.as_str())),
					)
					.set(component_type.eq(component.component_type.as_str()))
					.execute(conn)
					.with_context("unable to update device_components")?;
				} else {
					let new_component_id = gen_new_component_id(conn, internal_id)
						.with_context("unable to generate a new component ID")?;

					// Insert the component
					insert_into(device_components)
						.values(DeviceComponentNew {
							device_key_info_id: internal_id,
							component_id: Cow::from(new_component_id.as_str()),
							component_type: Cow::from(component.component_type.as_str()),
						})
						.execute(conn)
						.with_context("unable to insert into device_components")?;
				}
			}

			Ok(())
		})
		.with_context("unable to update the device entry")?;

	// Return the results
	Ok(json!({ "deviceId": device.clone() }))
}

#[post("/valueExists/<column_id>", data = "<query>")]
pub fn get_data_value_exists(
	mut conn: DbConn,
	column_id: i32,
	query: Json<ValueExistsQuery>,
) -> Result<JsonValue, Error> {
	let result = data_value_exists(
		&mut conn.0,
		column_id,
		query.device_id.as_deref(),
		query.value.as_str(),
	)?;

	Ok(json!({ "exists": result }))
}
