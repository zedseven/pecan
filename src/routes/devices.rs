// Uses
use std::borrow::Cow;

use anyhow::{anyhow, Context, Error};
use chrono::Utc;
use diesel::{
	dsl::exists,
	insert_into,
	select,
	BelongingToDsl,
	Connection,
	ExpressionMethods,
	GroupedBy,
	QueryDsl,
	RunQueryDsl,
};
use rocket::{get, post, response::status::BadRequest, routes, Route};
use rocket_contrib::{
	json,
	json::{Json, JsonValue},
};

use super::Routable;
use crate::{
	db::{functions::last_insert_rowid, models::*, schema, DbConn},
	util::gen_new_id,
};

/// The route for this section.
pub(super) struct DevicesApi;
impl Routable for DevicesApi {
	const PATH: &'static str = "/devices";
	const ROUTES: &'static dyn Fn() -> Vec<Route> = &|| {
		routes![
			get_columns,
			get_recent_entries,
			get_device,
			create_new_device
		]
	};
}

// Type Definitions
#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SubmittedDeviceInfo {
	location_id: i32,
	column_data: Vec<SubmittedColumnData>,
}
#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SubmittedColumnData {
	column_definition_id: i32,
	data_value: String,
}

// TODO: Bad return value
/// Fetches the column definitions.
#[get("/columns")]
pub fn get_columns(mut conn: DbConn) -> Result<JsonValue, BadRequest<&'static str>> {
	use schema::column_definitions::dsl::*;

	let column_definition_results = column_definitions
		.load::<ColumnDefinition>(&mut conn.0)
		.expect("unable to load the column definitions");

	Ok(json!(column_definition_results))
}

/// Fetches a device by ID.
#[get("/<device>")]
pub fn get_device(mut conn: DbConn, device: String) -> Result<JsonValue, BadRequest<&'static str>> {
	use schema::{device_key_info::dsl::*, locations::dsl::*};

	let device_key_info_results = device_key_info
		.filter(schema::device_key_info::dsl::device_id.eq(device.as_str()))
		.inner_join(locations)
		.select(DEVICE_INFO)
		.get_result::<DeviceInfo>(&mut conn.0)
		.expect("unable to load device info");

	let device_data_results = DeviceData::belonging_to(&device_key_info_results)
		.get_results::<DeviceData>(&mut conn.0)
		.expect("unable to load the device data");

	Ok(json!((device_key_info_results, device_data_results)))
}

/// Fetches the most recently-updated `count` entries.
#[get("/recent/<count>")]
pub fn get_recent_entries(
	mut conn: DbConn,
	count: u32,
) -> Result<JsonValue, BadRequest<&'static str>> {
	use schema::{column_definitions::dsl::*, device_key_info::dsl::*, locations::dsl::*};

	if count > 100 {
		return Err(BadRequest(Some("The count is too high.")));
	}

	let column_definition_results = column_definitions
		.load::<ColumnDefinition>(&mut conn.0)
		.expect("unable to load the column definitions");

	let device_key_info_results = device_key_info
		.order(last_updated.desc())
		.limit(i64::from(count))
		.inner_join(locations)
		.select(DEVICE_INFO)
		.load::<DeviceInfo>(&mut conn.0)
		.expect("unable to load device info");

	let device_data_results = DeviceData::belonging_to(&device_key_info_results)
		.load::<DeviceData>(&mut conn.0)
		.expect("unable to load the device data")
		.grouped_by(&device_key_info_results);

	let device_results = device_key_info_results
		.into_iter()
		.zip(device_data_results)
		.collect::<Vec<_>>();

	Ok(json!({ "columnDefinitions": column_definition_results, "deviceResults": device_results }))
}

/// Adds a new device to the database.
#[post("/add", data = "<device_info>")]
pub fn create_new_device(
	mut conn: DbConn,
	device_info: Json<SubmittedDeviceInfo>,
) -> Result<String, BadRequest<&'static str>> {
	use schema::{device_data::dsl::*, device_key_info::dsl::*, locations::dsl::*};

	// Verify the new location
	if !select(exists(
		locations.filter(schema::locations::dsl::id.eq(device_info.location_id)),
	))
	.get_result::<bool>(&mut conn.0)
	.expect("unable to query the database for location existence")
	{
		return Err(BadRequest(Some("Invalid location.")));
	}

	// Generate a new device ID
	let new_device_id = gen_new_id(&mut conn);

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
				.with_context(|| "unable to insert into device_key_info")?;

			// Fetch the new device's internal ID (SQLite doesn't support `RETURNING`
			// clauses)
			let new_device_key_info_id = select(last_insert_rowid())
				.get_result::<i32>(conn)
				.with_context(|| "unable to get the last insert rowid")?;

			// SQLite returns a value of 0 if no `INSERT` queries have been run on the
			// connection - it should never happen, but this is just a sanity check
			if new_device_key_info_id <= 0 {
				return Err(anyhow!(
					"no insert queries have been run on this connection - something is wrong"
				));
			}

			// Insert the device column data
			insert_into(device_data)
				.values(
					device_info
						.column_data
						.iter()
						.map(|data| DeviceDataNew {
							device_key_info_id: new_device_key_info_id,
							column_definition_id: data.column_definition_id,
							data_value: Some(Cow::from(data.data_value.as_str())),
						})
						.collect::<Vec<_>>(),
				)
				.execute(conn)
				.with_context(|| "unable to insert into device_data")?;

			Ok(())
		})
		.expect("unable to create the new device entry");

	Ok(new_device_id)
}
