// Uses
use diesel::SqliteConnection;
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
use rocket::{Build, Rocket};
use rocket_sync_db_pools::database;

// Modules
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
pub async fn init(rocket: Rocket<Build>) -> Rocket<Build> {
	let conn: DbConn = DbConn::get_one(&rocket)
		.await
		.expect("unable to get a database connection for initialisation");

	conn.run(|c| {
		c.run_pending_migrations(MIGRATIONS)
			.expect("failed to run embedded database migrations");
	})
	.await;

	rocket
}
