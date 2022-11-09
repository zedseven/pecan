// Uses
use std::fmt::Debug;

use diesel::{sql_query, RunQueryDsl, SqliteConnection};
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
use rocket::{Build, Rocket};
use rocket_sync_db_pools::database;

// Modules
pub mod change_log;
pub mod enums;
pub mod functions;
pub mod models;
pub mod schema;
pub mod util;

/// A database connection, provided by [diesel].
#[database("sqlite_database")]
pub struct DbConn(pub(crate) SqliteConnection);

// Embed the database migrations so they can be run on startup, straight from
// the compiled binary.
pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!("migrations");

/// Initialises the database and runs all necessary migrations.
pub async fn init(rocket: Rocket<Build>) -> Result<Rocket<Build>, Rocket<Build>> {
	// Get a database connection from the pool
	let conn: DbConn = if let Some(c) = DbConn::get_one(&rocket).await {
		c
	} else {
		eprintln!("unable to get a database connection for initialisation");
		return Err(rocket);
	};

	// Disable foreign key constraints before running pending migrations - this is
	// because some data migrations will throw errors if foreign key constraints are
	// in effect, even if in the end the data will be valid.
	// According to https://www.sqlite.org/foreignkeys.html#fk_enable, these statements do not
	// work within a multi-statement transaction so they cannot be included in the
	// migrations themselves.
	if conn
		.run(|c| {
			is_err_display_error(
				sql_query("PRAGMA foreign_keys = OFF;").execute(c),
				"unable to disable foreign key constraints",
			)
		})
		.await
	{
		return Err(rocket);
	}

	// Run the pending migrations
	if conn
		.run(|c| {
			is_err_display_error(
				c.run_pending_migrations(MIGRATIONS),
				"failed to run embedded database migrations",
			)
		})
		.await
	{
		return Err(rocket);
	}

	// Enable foreign key constraints after running pending migrations
	if conn
		.run(|c| {
			is_err_display_error(
				sql_query("PRAGMA foreign_keys = ON;").execute(c),
				"unable to enable foreign key constraints",
			)
		})
		.await
	{
		return Err(rocket);
	}

	Ok(rocket)
}

fn is_err_display_error<T, E>(result: Result<T, E>, message: &str) -> bool
where
	E: Debug,
{
	match result {
		Ok(_) => false,
		Err(e) => {
			eprintln!("{message}: {e:?}");
			true
		}
	}
}
