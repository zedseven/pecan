// Uses
use diesel::{dsl::exists, select, ExpressionMethods, QueryDsl, RunQueryDsl, SqliteConnection};

use crate::{
	db::schema,
	error::{Context, Error},
	id_gen::{gen_new_id, NumericAscii},
};

/// Generates a new device ID, and ensures it's not already in use.
pub fn gen_new_device_id(conn: &mut SqliteConnection) -> Result<String, Error> {
	const LENGTH: usize = 6;

	gen_new_id(NumericAscii, LENGTH, |new_id| {
		use schema::device_key_info::dsl::*;

		select(exists(device_key_info.filter(device_id.eq(new_id))))
			.get_result::<bool>(conn)
			.with_context("unable to query the database for a device ID")
	})
}

/// Generates a new component ID, and ensures it's not already in use with
/// `device_id`.
pub fn gen_new_component_id(conn: &mut SqliteConnection, device_id: i32) -> Result<String, Error> {
	const LENGTH: usize = 3;

	gen_new_id(NumericAscii, LENGTH, |new_id| {
		use schema::device_components::dsl::*;

		select(exists(
			device_components
				.filter(device_key_info_id.eq(device_id))
				.filter(component_id.eq(new_id)),
		))
		.get_result::<bool>(conn)
		.with_context("unable to query the database for a component ID")
	})
}
