// Uses
use diesel::{
	insert_into,
	Connection,
	ExpressionMethods,
	OptionalExtension,
	QueryDsl,
	RunQueryDsl,
	SqliteConnection,
};

use crate::{
	db::{models::UserNew, schema, util::fetch_new_rowid_on},
	error::{Context, Error},
};

pub fn create_user_if_new(conn: &mut SqliteConnection, user_new: UserNew) -> Result<i32, Error> {
	conn.transaction::<_, Error, _>(|tc| {
		use schema::user_info::dsl::*;

		// Check for the existing user
		let existing_id = user_info
			.filter(unique_identifier.eq(&user_new.unique_identifier))
			.select(id)
			.get_result::<i32>(tc)
			.optional()
			.with_context("failed to query the database for user existence")?;

		// If they already exist, simply return the found ID
		if let Some(id_value) = existing_id {
			return Ok(id_value);
		}

		// Else, create the new user
		insert_into(user_info)
			.values(user_new)
			.execute(tc)
			.with_context("unable to insert into user_info")?;

		// Return the new user ID
		fetch_new_rowid_on(tc).with_context("unable to get the new user_info id")
	})
}
