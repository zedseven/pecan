//! For things that are directly related to database operations, at a lower
//! level.

// Uses
use diesel::{
	dsl::{exists, not},
	select,
	ExpressionMethods,
	QueryDsl,
	RunQueryDsl,
	SqliteConnection,
};

use super::{functions::last_insert_rowid, schema};
use crate::error::{Context, Error};

/// Fetch the last-inserted `ROWID` on the connection, and ensure it's valid.
///
/// This is required because SQLite doesn't support `RETURNING` clauses.
///
/// Returns an error on a database problem or invalid `ROWID`.
pub fn fetch_new_rowid_on(conn: &mut SqliteConnection) -> Result<i32, Error> {
	// Use the `last_insert_rowid` SQLite function to fetch the rowid
	let new_rowid = select(last_insert_rowid())
		.get_result::<i32>(conn)
		.with_context("unable to get the last insert rowid")?;

	// SQLite returns a value of 0 if no `INSERT` queries have been run on the
	// connection
	if new_rowid <= 0 {
		return Err("no insert queries have been run on this connection")?;
	}

	Ok(new_rowid)
}

/// Checks if a `device_data` value exists already.
pub fn data_value_exists(
	conn: &mut SqliteConnection,
	column_id: i32,
	device_opt: Option<&str>,
	value: &str,
) -> Result<bool, Error> {
	use schema::{device_data::dsl::*, device_key_info::dsl::*};

	let mut sub_query = device_data
		.inner_join(device_key_info)
		.filter(deleted.eq(false))
		.filter(column_definition_id.eq(column_id))
		.filter(data_value.eq(value))
		.into_boxed();

	// Ensure that the value for the current device doesn't count as a match
	// (we only care about *other* devices that have the same value)
	if let Some(device) = device_opt {
		sub_query = sub_query.filter(not(device_id.eq(device)));
	}

	select(exists(sub_query))
		.get_result::<bool>(conn)
		.with_context("unable to query the database for data value existence")
}
