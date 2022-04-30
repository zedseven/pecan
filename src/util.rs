// Uses
use diesel::{dsl::exists, select, ExpressionMethods, QueryDsl, RunQueryDsl};
use rand::{
	distributions::{Distribution, Uniform},
	thread_rng,
};

use crate::db::{schema, DbConn};

/// Generates a new device ID, and ensures it's not already in use.
///
/// This is based on the [Microsoft Alphabet] - the set of characters they use
/// to generate product keys, designed to minimise ambiguity and prevent words
/// from being spelled by chance.
///
/// [Microsoft Alphabet]: https://www.techtalkz.com/threads/alphabet-letters-not-used-in-microsoft-product-keys.82675/#post-349424
pub fn gen_new_id(conn: &mut DbConn) -> String {
	const PREFIX: &str = "T-"; // TODO: Make this configurable
	const LENGTH: usize = 8;
	const CHAR_SET: &[char] = &[
		'2', '3', '4', '6', '7', '9', 'C', 'D', 'F', 'G', 'H', 'J', 'K', 'M', 'P', 'Q', 'R', 'T',
		'V', 'W', 'X', 'Y',
	];

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
