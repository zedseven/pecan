// Uses
use diesel::{dsl::exists, select, ExpressionMethods, QueryDsl, RunQueryDsl, SqliteConnection};
use rand::{
	distributions::{Distribution, Uniform},
	thread_rng,
};

use crate::{
	db::schema,
	error::{Context, Error},
};

/// Generates a new device ID, and ensures it's not already in use.
pub fn gen_new_device_id(conn: &mut SqliteConnection) -> Result<String, Error> {
	const LENGTH: usize = 6;

	gen_new_id(conn, LENGTH, |conn, new_id| {
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

	gen_new_id(conn, LENGTH, |conn, new_id| {
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

fn gen_new_id<P>(conn: &mut SqliteConnection, length: usize, id_in_use: P) -> Result<String, Error>
where
	P: Fn(&mut SqliteConnection, &str) -> Result<bool, Error>,
{
	const CHAR_SET: &[char] = &['0', '1', '2', '3', '4', '5', '6', '7', '8', '9'];

	let uniform = Uniform::from(0..CHAR_SET.len());
	let mut rng = thread_rng();

	let mut new_id = String::with_capacity(length);
	loop {
		// Generate the new ID
		new_id.clear();
		for _ in 0..length {
			new_id.push(CHAR_SET[uniform.sample(&mut rng)]);
		}

		// Verify that it's unused
		if !id_in_use(conn, new_id.as_str())? {
			break;
		}
	}

	Ok(new_id)
}
