//! For things that are directly related to database operations, at a lower
//! level.

// Uses
use diesel::{select, RunQueryDsl, SqliteConnection};

use super::functions::last_insert_rowid;
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
