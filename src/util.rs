// Uses
use diesel::{dsl::exists, select, ExpressionMethods, QueryDsl, RunQueryDsl};
use rand::{
	distributions::{Distribution, Uniform},
	thread_rng,
};

use crate::db::{schema, DbConn};

/// Generates a new device ID, and ensures it's not already in use.
pub fn gen_new_id(conn: &mut DbConn) -> String {
	const PREFIX: &str = "T-"; // TODO: Make this configurable
	const LENGTH: usize = 6;
	const CHAR_SET: &[char] = &['0', '1', '2', '3', '4', '5', '6', '7', '8', '9'];

	let uniform = Uniform::from(0..CHAR_SET.len());
	let mut rng = thread_rng();

	let mut result = String::with_capacity(LENGTH + PREFIX.len());
	loop {
		use schema::device_key_info::dsl::*;

		// Generate the new ID
		result.clear();
		result.push_str(PREFIX);
		for _ in 0..LENGTH {
			result.push(CHAR_SET[uniform.sample(&mut rng)]);
		}

		// Verify that it's unused
		if !select(exists(
			device_key_info.filter(device_id.eq(result.as_str())),
		))
		.get_result::<bool>(&mut conn.0)
		.expect("unable to query the database for a device ID")
		{
			break;
		}
	}

	result
}
