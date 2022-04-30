// Uses
use diesel::{BelongingToDsl, ExpressionMethods, GroupedBy, QueryDsl, RunQueryDsl};
use rocket::{get, response::status::BadRequest, routes, Route};
use rocket_contrib::{json, json::JsonValue};

use super::Routable;
use crate::db::{
	models::{ColumnDefinition, DeviceData, DeviceInfo, DEVICE_INFO},
	schema,
	DbConn,
};

/// The route for this section.
pub(super) struct Api;
impl Routable for Api {
	const PATH: &'static str = "/api";
	const ROUTES: &'static dyn Fn() -> Vec<Route> = &|| routes![get_recent_entries];
}

// type TestType = (
// 	schema::device_key_info::device_id,
// 	schema::locations::location,
// );
//
// const TEST: TestType = ();

#[get("/recent/<count>")]
pub fn get_recent_entries(
	mut conn: DbConn,
	count: u32,
) -> Result<JsonValue, BadRequest<&'static str>> {
	use schema::{
		column_definitions::dsl::column_definitions,
		device_key_info::dsl::{device_key_info, last_updated},
		locations::dsl::locations,
	};

	if count > 100 {
		return Err(BadRequest(Some("The count is too high.")));
	}

	let column_definition_results = column_definitions
		.load::<ColumnDefinition>(&mut conn.0)
		.expect("unable to load the column definitions");

	// let location_results = locations
	// 	.load::<ColumnDefinition>(&conn.0)
	// 	.expect("unable to load the column definitions");

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
