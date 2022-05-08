// Uses
use diesel::SqliteConnection;
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
use rocket::Rocket;
use rocket_contrib::database;

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
pub fn init(rocket: Rocket) -> Result<Rocket, Rocket> {
	let mut conn: DbConn =
		DbConn::get_one(&rocket).expect("unable to get a database connection for initialisation");

	conn.0
		.run_pending_migrations(MIGRATIONS)
		.expect("failed to run embedded database migrations");

	Ok(rocket)
}
