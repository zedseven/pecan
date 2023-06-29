// Uses
use std::borrow::Cow;

use base64::{engine::general_purpose::STANDARD as BASE64_STANDARD, Engine};
use diesel::{
	dsl::exists,
	insert_into,
	query_builder::{BoxedSqlQuery, SqlQuery},
	result::OptionalExtension,
	select,
	sql_query,
	sql_types::{Integer, Nullable, Text},
	sqlite::Sqlite,
	update,
	upsert::excluded,
	BelongingToDsl,
	Connection,
	ExpressionMethods,
	GroupedBy,
	JoinOnDsl,
	NullableExpressionMethods,
	QueryDsl,
	RunQueryDsl,
	SqliteConnection,
};
use rocket::{
	data::ByteUnit,
	get,
	post,
	routes,
	serde::json::{json, Json, Value as JsonValue},
	Route,
	State,
};

use super::Routable;
use crate::{
	auth::AuthedUser,
	config::AppConfig,
	db::{change_log::*, models::*, schema, util::data_value_exists, DbConn},
	error::{Context, Error, UserError},
	routes::file_from_memory::FileFromMemory,
	util::{gen_new_attachment_id, gen_new_component_id, gen_new_device_id},
};

/// The route for this section.
pub(super) struct DevicesApi;
impl Routable for DevicesApi {
	const PATH: &'static str = "/devices";
	const ROUTES: &'static dyn Fn() -> Vec<Route> = &|| {
		routes![
			get_definitions,
			search_devices_default,
			search_devices,
			get_device,
			checkout_device,
			create_device,
			update_device,
			get_attachment,
			get_device_exists,
			get_data_value_exists
		]
	};
}

// Type Definitions
#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdatedDeviceInfo {
	location_id: i32,
	column_data: Vec<SubmittedColumnData>,
	components:  Vec<UpdatedDeviceComponent>,
	attachments: Vec<UpdatedDeviceAttachment>,
}
#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdatedDeviceComponent {
	component_id:   Option<String>,
	component_type: String,
}
#[derive(Deserialize)]
#[serde(untagged)]
pub enum UpdatedDeviceAttachment {
	#[serde(rename_all = "camelCase")]
	New {
		description: String,
		file_name:   String,
		file_data:   String,
	},
	#[serde(rename_all = "camelCase")]
	Existing {
		attachment_id: String,
		description:   String,
	},
}
#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SubmittedSearchQuery {
	device_id:   String,
	location_id: Option<i32>,
	column_data: Vec<SubmittedColumnData>,
}
#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SubmittedColumnData {
	column_definition_id: i32,
	data_value:           String,
}
#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CheckoutInfo {
	device_id:   String,
	location_id: i32,
}
#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ValueExistsQuery {
	device_id: Option<String>,
	value:     String,
}

/// Fetches the column definitions and locations.
#[get("/definitions")]
pub async fn get_definitions(
	config: &State<AppConfig>,
	user: &AuthedUser,
	conn: DbConn,
) -> Result<JsonValue, Error> {
	let max_attachment_size = config.max_attachment_size.as_u64();
	let user_clone = user.0.clone();
	conn.run(move |c| {
		// Uses
		use schema::{
			column_definitions::dsl::*,
			column_possible_values::dsl::*,
			locations::dsl::*,
		};

		// Load the data
		let column_definition_results = column_definitions
			.left_join(
				column_possible_values
					.on(default_value_id.eq(schema::column_possible_values::dsl::id.nullable())),
			)
			.order_by(ordering_key)
			.then_order_by(schema::column_definitions::dsl::id)
			.select(COLUMN_DEFINITION())
			.load::<ColumnDefinitionSelected<'_>>(c)
			.with_context("unable to load the column definitions")?;

		let possible_values_results = ColumnPossibleValue::belonging_to(&column_definition_results)
			.order_by(value)
			.load::<ColumnPossibleValue<'_>>(c)
			.with_context("unable to load the column possible values")?
			.grouped_by(&column_definition_results);

		let column_results = column_definition_results
			.into_iter()
			.zip(possible_values_results)
			.collect::<Vec<_>>();

		let location_results = locations
			.order_by(schema::locations::dsl::name)
			.load::<LocationDefinition<'_>>(c)
			.with_context("unable to load the locations")?;

		Ok(
			json!({ "currentUser": user_clone, "columnDefinitions": column_results, "locations": location_results, "maxAttachmentSize": max_attachment_size }),
		)
	})
	.await
}

/// Fetches a device by ID.
#[get("/get/<device>")]
pub async fn get_device(
	_user: &AuthedUser,
	conn: DbConn,
	device: String,
) -> Result<JsonValue, Error> {
	conn.run(move |c| {
		let (
			device_key_info_results,
			device_data_results,
			device_component_results,
			device_attachment_results,
			device_change_results,
		) = load_device_info(c, device.as_str())?;

		// Return the results
		// This odd return format is to match how the data is returned in search
		// results.
		Ok(json!({
			"deviceResults": (device_key_info_results, device_data_results),
			"deviceComponents": device_component_results,
			"deviceAttachments": device_attachment_results,
			"deviceChanges": device_change_results,
		}))
	})
	.await
}

pub type CompleteDeviceInfo<'a> = (
	DeviceInfo<'a>,
	Vec<DeviceData<'a>>,
	Vec<DeviceComponent<'a>>,
	Vec<DeviceAttachmentMetadata<'a>>,
	Vec<DeviceChangeDisplay<'a>>,
);
/// Fetches a device by ID.
pub fn load_device_info<'a, 'b>(
	conn: &'a mut SqliteConnection,
	device: &str,
) -> Result<CompleteDeviceInfo<'b>, Error> {
	// Uses
	use schema::{
		column_definitions::dsl::*,
		device_attachments::dsl::*,
		device_changes::dsl::*,
		device_components::dsl::*,
		device_data::dsl::*,
		device_key_info::dsl::*,
		locations::dsl::*,
		user_info::dsl::*,
	};

	// Load from the database
	let device_key_info_result = device_key_info
		.filter(device_id.eq(device))
		.inner_join(locations)
		.select((
			schema::device_key_info::dsl::id,
			device_id,
			location_id,
			schema::locations::dsl::name,
			device_changes
				.select(timestamp)
				.filter(
					schema::device_changes::dsl::device_key_info_id
						.eq(schema::device_key_info::dsl::id),
				)
				.order_by(timestamp.desc())
				.limit(1)
				.single_value()
				.assume_not_null(),
		))
		.get_result::<DeviceInfo<'_>>(conn)
		.optional()
		.with_context("unable to load device info")?;
	if device_key_info_result.is_none() {
		return Err(UserError::NotFound("Invalid device ID.").into());
	}
	let device_key_info_result = device_key_info_result.unwrap();

	let device_data_results = DeviceData::belonging_to(&device_key_info_result)
		.inner_join(column_definitions)
		.order_by(ordering_key)
		.then_order_by(column_definition_id)
		.select(DEVICE_DATA)
		.get_results::<DeviceData<'_>>(conn)
		.with_context("unable to load the device data")?;

	let device_component_results = DeviceComponent::belonging_to(&device_key_info_result)
		.order_by(component_type)
		.get_results::<DeviceComponent<'_>>(conn)
		.with_context("unable to load the device components")?;

	let device_attachment_results = DeviceAttachmentMetadata::belonging_to(&device_key_info_result)
		.order_by(file_name)
		.select(DEVICE_ATTACHMENT_METADATA)
		.get_results::<DeviceAttachmentMetadata<'_>>(conn)
		.with_context("unable to load the device attachments")?;

	let device_change_results = DeviceChange::belonging_to(&device_key_info_result)
		.left_join(user_info)
		.select(DEVICE_CHANGE())
		.order_by(timestamp.desc())
		.get_results::<DeviceChangeDisplay<'_>>(conn)
		.with_context("unable to load the device changes")?;

	// Return the results
	Ok((
		device_key_info_result,
		device_data_results,
		device_component_results,
		device_attachment_results,
		device_change_results,
	))
}

fn perform_search(
	conn: &mut SqliteConnection,
	device_key_info_query: BoxedSqlQuery<'_, Sqlite, SqlQuery>,
) -> Result<JsonValue, Error> {
	use schema::{column_definitions::dsl::*, device_data::dsl::*};

	let device_key_info_results = device_key_info_query
		.load::<DeviceInfoByName<'_>>(conn)
		.with_context("unable to load device info")?
		.drain(..)
		.map(Into::into)
		.collect::<Vec<DeviceInfo<'_>>>();
	// dbg!(&device_key_info_results);

	// Collect the device data
	let device_data_results = DeviceData::belonging_to(&device_key_info_results)
		.inner_join(column_definitions)
		.order_by(ordering_key)
		.then_order_by(column_definition_id)
		.select(DEVICE_DATA)
		.load::<DeviceData<'_>>(conn)
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

/// Fetches the results for the default landing page.
#[post("/search/default")]
pub async fn search_devices_default(_user: &AuthedUser, conn: DbConn) -> Result<JsonValue, Error> {
	conn.run(move |c| {
		let search_sql = include_str!(concat!(
			env!("CARGO_MANIFEST_DIR"),
			"/src/sql/default_search.sql"
		));
		let device_key_info_query = sql_query(search_sql).into_boxed();

		perform_search(c, device_key_info_query)
	})
	.await
}

/// Performs a search query for a user.
#[post("/search", data = "<search_query>")]
pub async fn search_devices(
	_user: &AuthedUser,
	conn: DbConn,
	search_query: Json<SubmittedSearchQuery>,
) -> Result<JsonValue, Error> {
	// Check if any column values were specified for searching
	let search_column_data_is_present = search_query
		.column_data
		.iter()
		.any(|column| !column.data_value.is_empty());

	conn.run(move |c| {
		// Search the device key info
		// This whole thing is *extremely* ugly. This is because Diesel doesn't support
		// boxed sub-queries, because the boxing operation can't know that it will only
		// be used in a sub-query where the referenced parent column is valid.
		// The boxing operation is required because we dynamically add filter conditions
		// to the query.
		let mut search_sql = String::from(
			"SELECT
			    dki.id,
			    dki.device_id,
			    dki.location_id,
			    l.name AS location,
			    (
					SELECT
						dc.timestamp
					FROM device_changes AS dc
					WHERE
						dc.device_key_info_id = dki.id AND
						dc.done_automatically = 0
					ORDER BY dc.timestamp DESC
					LIMIT 1
				) AS last_updated
			FROM device_key_info AS dki
			INNER JOIN locations AS l ON l.id = dki.location_id
			WHERE
			    dki.device_id LIKE ?
			    AND (? IS NULL OR dki.location_id = ?)\n",
		);
		let mut bind_params = Vec::new();
		if search_column_data_is_present {
			search_sql.push_str(
				"AND ? = (SELECT
	               COUNT(dd.id)
	           FROM device_data AS dd
	           WHERE dd.device_key_info_id = dki.id AND (",
			);
			let mut first_entry = true;
			for column_query in &search_query.column_data {
				if column_query.data_value.is_empty() {
					continue;
				}
				if !first_entry {
					search_sql.push_str(" OR ");
				}
				search_sql.push_str("(dd.column_definition_id = ? AND dd.data_value LIKE ?)");
				bind_params.push((
					column_query.column_definition_id,
					format!("%{}%", column_query.data_value.as_str()),
				));
				first_entry = false;
			}
			search_sql.push_str("))\n");
		}
		search_sql.push_str("ORDER BY last_updated DESC");
		// dbg!(&search_sql);
		let mut device_key_info_query = sql_query(search_sql)
			.into_boxed()
			.bind::<Text, _>(format!("%{}%", search_query.device_id.as_str()))
			.bind::<Nullable<Integer>, _>(search_query.location_id)
			.bind::<Nullable<Integer>, _>(search_query.location_id);
		if search_column_data_is_present {
			device_key_info_query =
				device_key_info_query.bind::<Integer, _>(bind_params.len() as i32);
		}
		for (bind_column_definition_id, bind_data_value_search) in bind_params {
			device_key_info_query = device_key_info_query
				.bind::<Integer, _>(bind_column_definition_id)
				.bind::<Text, _>(bind_data_value_search);
		}

		perform_search(c, device_key_info_query)
	})
	.await
}

#[post("/checkout", data = "<checkout_info>")]
pub async fn checkout_device(
	user: &AuthedUser,
	conn: DbConn,
	checkout_info: Json<CheckoutInfo>,
) -> Result<JsonValue, Error> {
	let user_id_value = user.0.id;
	conn.run(move |c| {
		// Uses
		use schema::{device_key_info::dsl::*, locations::dsl::*};

		// Verify the new location
		let location_name = locations
			.filter(schema::locations::dsl::id.eq(checkout_info.location_id))
			.select(schema::locations::dsl::name)
			.get_result::<String>(c)
			.optional()
			.with_context("unable to query the database for location existence")?;
		let location_name = if let Some(location_name_value) = location_name {
			location_name_value
		} else {
			return Err(UserError::BadRequest("Invalid location.").into());
		};

		c.transaction::<_, Error, _>(|tc| {
			// Get the old value
			let old_device_key_info = device_key_info
				.filter(device_id.eq(checkout_info.device_id.as_str()))
				.get_result::<DeviceKeyInfo<'_>>(tc)
				.optional()
				.with_context("unable to query the database for device_key_info existence")?;
			let old_device_key_info = if let Some(old_device_key_info_value) = old_device_key_info {
				old_device_key_info_value
			} else {
				return Err(UserError::BadRequest("Invalid device ID.").into());
			};

			// Update the device entry
			update(
				device_key_info.filter(schema::device_key_info::dsl::id.eq(old_device_key_info.id)),
			)
			.set(location_id.eq(checkout_info.location_id))
			.execute(tc)
			.with_context("unable to update device_key_info")?;

			// Log the change in the database
			if old_device_key_info.location_id != checkout_info.location_id {
				let diff = DeviceDiff {
					device_key_info:    Some(DeviceKeyInfoDiff::Edit(DeviceKeyInfoDiffData {
						location_id: Some(checkout_info.location_id),
					})),
					device_data:        None,
					device_components:  None,
					device_attachments: None,
				};

				log_change(tc, old_device_key_info.id, user_id_value, &diff)
					.with_context("unable to log device change")?;
			}

			Ok(())
		})
		.with_context("unable to update the device entry")?;

		// Return the results
		Ok(json!({
			"deviceId": checkout_info.device_id.clone(),
			"locationId": checkout_info.location_id,
			"locationName": location_name
		}))
	})
	.await
}

/// Adds a new device to the database.
#[post("/create", data = "<device_info>")]
pub async fn create_device<'a>(
	config: &State<AppConfig>,
	user: &AuthedUser,
	conn: DbConn,
	device_info: Json<UpdatedDeviceInfo>,
) -> Result<JsonValue, Error> {
	upsert_device(
		config.max_attachment_size,
		conn,
		None,
		device_info,
		user.0.id,
	)
	.await
}

/// Updates a device's data.
#[post("/update/<device>", data = "<device_info>")]
pub async fn update_device(
	config: &State<AppConfig>,
	user: &AuthedUser,
	conn: DbConn,
	device: String,
	device_info: Json<UpdatedDeviceInfo>,
) -> Result<JsonValue, Error> {
	upsert_device(
		config.max_attachment_size,
		conn,
		Some(device),
		device_info,
		user.0.id,
	)
	.await
}

/// Inserts or updates device information, depending on if it's already present
/// in the database. This is the implementation for [`create_device`] and
/// [`update_device`].
async fn upsert_device(
	max_attachment_size: ByteUnit,
	conn: DbConn,
	device: Option<String>,
	device_info: Json<UpdatedDeviceInfo>,
	user_id_value: i32,
) -> Result<JsonValue, Error> {
	conn.run(move |c| {
		// Uses
		use schema::{
			device_attachments::dsl::*,
			device_components::dsl::*,
			device_data::dsl::*,
			device_key_info::dsl::*,
			locations::dsl::*,
		};

		// Verify the new location
		if !select(exists(
			locations.filter(schema::locations::dsl::id.eq(device_info.location_id)),
		))
		.get_result::<bool>(c)
		.with_context("unable to query the database for location existence")?
		{
			return Err(UserError::NotFound("Invalid location.").into());
		}

		// Generate a new device ID if one wasn't provided
		let (is_new, prepared_device_id) = if let Some(provided_device_id) = device {
			(false, provided_device_id)
		} else {
			(
				true,
				gen_new_device_id(c).with_context("unable to generate a new device ID")?,
			)
		};

		// Technically there should be verification that duplicate values aren't being
		// submitted here, but it's already enforced by the frontend and it's not worth
		// the many additional queries right now
		// The same goes for not-null values

		// Begin the transaction
		c.transaction::<_, Error, _>(|tc| {
			let old_values = if is_new {
				None
			} else {
				// Pull the existing values
				let (
					device_key_info_result,
					device_data_results,
					device_component_results,
					device_attachment_results,
					_,
				) = load_device_info(tc, prepared_device_id.as_str())?;
				Some((
					device_key_info_result,
					device_data_results,
					device_component_results,
					device_attachment_results,
				))
			};

			let insertable_device_key_info = DeviceKeyInfoNew {
				device_id:   Cow::from(prepared_device_id.as_str()),
				location_id: device_info.location_id,
			};

			// Upsert the main device entry
			insert_into(device_key_info)
				.values(&insertable_device_key_info)
				.on_conflict(device_id)
				.do_update()
				.set((location_id.eq(excluded(location_id)),))
				.execute(tc)
				.with_context("unable to upsert into device_key_info")?;

			// Fetch the device's internal ID for use in the other queries
			let internal_id = device_key_info
				.filter(device_id.eq(prepared_device_id.as_str()))
				.select(schema::device_key_info::dsl::id)
				.get_result::<i32>(tc)
				.with_context(
					"unable to get the internal ID associated with the prepared device ID",
				)?;

			// Upsert the device column data
			let mut insertable_device_data = Vec::new();
			for column in &device_info.column_data {
				insertable_device_data.push(DeviceDataNew {
					device_key_info_id:   internal_id,
					column_definition_id: column.column_definition_id,
					data_value:           Cow::from(column.data_value.as_str()),
				});
			}

			for insertable_record in &insertable_device_data {
				insert_into(device_data)
					.values(insertable_record)
					.on_conflict((
						schema::device_data::dsl::device_key_info_id,
						column_definition_id,
					))
					.do_update()
					.set(data_value.eq(excluded(data_value)))
					.execute(tc)
					.with_context("unable to upsert into device_data")?;
			}

			// Upsert the device components
			let mut insertable_device_components = Vec::new();
			for component in &device_info.components {
				// Generate a new component ID if one wasn't provided
				let prepared_component_id =
					if let Some(provided_component_id) = component.component_id.clone() {
						provided_component_id
					} else {
						gen_new_component_id(tc, internal_id)
							.with_context("unable to generate a new component ID")?
					};

				insertable_device_components.push(DeviceComponentNew {
					device_key_info_id: internal_id,
					component_id:       Cow::from(prepared_component_id),
					component_type:     Cow::from(component.component_type.as_str()),
				});
			}

			for insertable_record in &insertable_device_components {
				// Upsert the component
				insert_into(device_components)
					.values(insertable_record)
					.on_conflict((
						schema::device_components::dsl::device_key_info_id,
						component_id,
					))
					.do_update()
					.set(component_type.eq(excluded(component_type)))
					.execute(tc)
					.with_context("unable to upsert into device_components")?;
			}

			// Update the device attachments
			let mut insertable_device_attachments = Vec::new();
			for attachment in &device_info.attachments {
				match attachment {
					UpdatedDeviceAttachment::New {
						description: provided_description,
						file_name: provided_file_name,
						file_data: provided_file_data,
					} => {
						// Generate a new attachment ID
						let new_attachment_id = gen_new_attachment_id(tc, internal_id)
							.with_context("unable to generate a new attachment ID")?;

						// Decode the Base64-encoded file data
						let binary_file_data =
							BASE64_STANDARD.decode(provided_file_data).map_err(|_| {
								Error::User(UserError::BadRequest("Invalid file data."))
							})?;

						// Ensure the file size is below the configured limit
						if binary_file_data.len() as u64 > max_attachment_size.as_u64() {
							return Err(Error::User(UserError::BadRequest(
								"File size is above the configured limit.",
							)));
						}

						let default_file_name =
							format!("{prepared_device_id}-{new_attachment_id}.bin");
						insertable_device_attachments.push(DeviceAttachmentUpsert::New(
							DeviceAttachmentNew {
								device_key_info_id: internal_id,
								attachment_id:      Cow::from(new_attachment_id),
								description:        Cow::from(provided_description.trim()),
								file_name:          if provided_file_name.trim().is_empty() {
									Cow::from(default_file_name)
								} else {
									Cow::from(provided_file_name.trim())
								},
								file_data:          binary_file_data,
							},
						));
					}
					UpdatedDeviceAttachment::Existing {
						attachment_id: provided_attachment_id,
						description: provided_description,
					} => {
						insertable_device_attachments.push(DeviceAttachmentUpsert::Existing(
							DeviceAttachmentExisting {
								device_key_info_id: internal_id,
								attachment_id:      Cow::from(provided_attachment_id.as_str()),
								description:        Cow::from(provided_description.as_str()),
							},
						));
					}
				}
			}

			for insertable_record in &insertable_device_attachments {
				match insertable_record {
					DeviceAttachmentUpsert::New(new_record) => {
						// Insert the attachment
						insert_into(device_attachments)
							.values(new_record)
							.execute(tc)
							.with_context("unable to insert into device_attachments")?;
					}
					DeviceAttachmentUpsert::Existing(DeviceAttachmentExisting {
						attachment_id: provided_attachment_id,
						description: provided_description,
						..
					}) => {
						// Update the attachment
						update(
							device_attachments
								.filter(
									schema::device_attachments::dsl::device_key_info_id
										.eq(internal_id),
								)
								.filter(attachment_id.eq(provided_attachment_id.as_ref())),
						)
						.set(description.eq(provided_description.as_ref()))
						.execute(tc)
						.with_context("unable to update device_attachments")?;
					}
				}
			}

			// Calculate the diff
			let change_diff = if let Some(before) = old_values {
				DeviceDiff::calculate_diff(
					&before,
					&(
						insertable_device_key_info,
						insertable_device_data,
						insertable_device_components,
						insertable_device_attachments,
					),
				)
			} else {
				Some(DeviceDiff::from(&(
					insertable_device_key_info,
					insertable_device_data,
					insertable_device_components,
					insertable_device_attachments,
				)))
			};

			if let Some(diff) = change_diff {
				log_change(tc, internal_id, user_id_value, &diff)
					.with_context("unable to log device change")?;
			}

			Ok(())
		})
		.with_context("unable to update the device entry")?;

		// Return the results
		Ok(json!({ "deviceId": prepared_device_id }))
	})
	.await
}

#[get("/attachment/<device>/<attachment>")]
pub async fn get_attachment(
	_user: &AuthedUser,
	conn: DbConn,
	device: String,
	attachment: String,
) -> Result<FileFromMemory, Error> {
	conn.run(move |c| {
		// Uses
		use schema::{device_attachments::dsl::*, device_key_info::dsl::*};

		device_key_info
			.inner_join(device_attachments)
			.filter(device_id.eq(device.as_str()))
			.filter(attachment_id.eq(attachment.as_str()))
			.select(DEVICE_ATTACHMENT)
			.get_result::<DeviceAttachment<'_>>(c)
			.optional()
			.with_context("unable to load a device attachment")?
			.map(|data| FileFromMemory::new(data.file_name.as_ref(), data.file_data))
			.ok_or_else(|| Error::User(UserError::NotFound("Attachment not found.")))
	})
	.await
}

#[post("/deviceExists/<device>")]
pub async fn get_device_exists(
	_user: &AuthedUser,
	conn: DbConn,
	device: String,
) -> Result<JsonValue, Error> {
	// Uses
	use schema::device_key_info::dsl::*;

	conn.run(move |c| {
		let result = select(exists(device_key_info.filter(device_id.eq(device))))
			.get_result::<bool>(c)
			.with_context("unable to query the database for device existence")?;

		Ok(json!({ "exists": result }))
	})
	.await
}

#[post("/valueExists/<column_id>", data = "<query>")]
pub async fn get_data_value_exists(
	_user: &AuthedUser,
	conn: DbConn,
	column_id: i32,
	query: Json<ValueExistsQuery>,
) -> Result<JsonValue, Error> {
	conn.run(move |c| {
		let result = data_value_exists(
			c,
			column_id,
			query.device_id.as_deref(),
			query.value.as_str(),
		)?;

		Ok(json!({ "exists": result }))
	})
	.await
}
