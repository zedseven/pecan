// Uses
use std::borrow::Cow;

use chrono::Utc;
use diesel::{
	dsl::exists,
	insert_into,
	select,
	Connection,
	ExpressionMethods,
	QueryDsl,
	RunQueryDsl,
	SqliteConnection,
};

use crate::{
	auth::get_token_valid_duration,
	db::{models::TokenNew, schema},
	error::{Context, Error},
	id_gen::{gen_new_id, Base64},
};

/// Generates a new token, inserts it into the database, and returns the new
/// token's value.
pub fn generate_token_for_user(
	conn: &mut SqliteConnection,
	token_user: &str,
	token_valid_days: u32,
) -> Result<String, Error> {
	// Uses
	use schema::tokens::dsl::*;

	// Generate the new token
	let token_value =
		generate_token_value(conn).with_context("unable to generate a new token value")?;

	// Insert it into the database
	conn.transaction::<_, Error, _>(|tc| {
		// Insert the new device entry
		insert_into(tokens)
			.values(TokenNew {
				user: Cow::from(token_user),
				value: Cow::from(token_value.as_str()),
				expires: (Utc::now() + get_token_valid_duration(token_valid_days)).naive_utc(),
			})
			.execute(tc)
			.with_context("unable to insert into tokens")
	})?;

	// Return the new value
	Ok(token_value)
}

// It's highly unnecessary to verify that the token is unused, but I do it here
// because it's relatively inexpensive and I'd rather be 100% sure there will be
// no duplicates, even with poor OS RNG.
fn generate_token_value(conn: &mut SqliteConnection) -> Result<String, Error> {
	const TOKEN_BITS: usize = 256;
	const FULL_U8_POSSIBLE_VALUES: usize = 256;
	const LENGTH: usize =
		(FULL_U8_POSSIBLE_VALUES / Base64::RANGE as usize) * (TOKEN_BITS / u8::BITS as usize);

	gen_new_id(Base64, LENGTH, |new_id| {
		use schema::tokens::dsl::*;

		select(exists(tokens.filter(value.eq(new_id))))
			.get_result::<bool>(conn)
			.with_context("unable to query the database for an existing token")
	})
}
