// Uses
use std::{
	ffi::OsStr,
	fs::File,
	io::{stderr, stdout, Write},
	path::Path,
	process::Command,
};

use chrono::Utc;
use which::{which, Result as WhichResult};

// Constants
const VERSION_FILE: &str = "VERSION.txt";
const COMMIT_HASH_LENGTH: usize = 8;

/// Builds the static site before starting up and prepares the version file.
fn main() {
	// Exit immediately if the `no-build` feature is passed. This is because for
	// some reason, Clippy tries to run the build script on Windows. On Linux, it
	// works fine.
	if cfg!(feature = "no-build") {
		return;
	}

	// Tell Cargo to rebuild Svelte if anything changed in its source directory
	println!(concat!(
		"cargo:rerun-if-changed=",
		env!("CARGO_MANIFEST_DIR"),
		"/web/src"
	));
	println!(concat!(
		"cargo:rerun-if-changed=",
		env!("CARGO_MANIFEST_DIR"),
		"/web/static"
	));
	println!(concat!(
		"cargo:rerun-if-changed=",
		env!("CARGO_MANIFEST_DIR"),
		"/web/svelte.config.js"
	));

	// Tell Cargo to re-run this build script if a new commit is made
	println!("cargo:rerun-if-changed=.git/HEAD");

	// Set the build mode for Svelte based on the Rust build mode
	let build_mode = if cfg!(debug_assertions) {
		"development"
	} else {
		"production"
	};

	// Provide other build metadata
	let build_version = concat!('v', env!("CARGO_PKG_VERSION"));
	let build_date = Utc::now()
		.naive_utc()
		.format("%Y-%m-%d %H:%M:%S UTC")
		.to_string();
	let commit_hash = get_git_commit_hash();
	let commit_hash_short = commit_hash
		.get(..COMMIT_HASH_LENGTH)
		.expect("unable to slice the commit hash");

	// Run the build command
	let svelte_output = new_command("npm")
		.expect("unable to locate an installed instance of NPM")
		.arg("run")
		.arg("build")
		.current_dir(concat!(env!("CARGO_MANIFEST_DIR"), "/web"))
		.env("BUILD_MODE", build_mode)
		.env("BUILD_VERSION", build_version)
		.env("BUILD_COMMIT", commit_hash_short)
		.env("BUILD_DATE", build_date)
		.output()
		.expect("failed to execute npm");

	println!("Svelte build: {}", svelte_output.status);
	stdout()
		.write_all(&svelte_output.stdout)
		.expect("failed to write to stdout");
	stderr()
		.write_all(&svelte_output.stderr)
		.expect("failed to write to stderr");

	// Exit if the Svelte build failed
	assert!(svelte_output.status.success());

	// Output the version string for this build
	write_to_file(
		VERSION_FILE,
		format!("{build_version}-{commit_hash_short}").as_str(),
	);
}

/// Previously we just used `Command::new(binary_name)`, but that didn't work on
/// Windows for some stupid reason.
fn new_command<S>(binary_name: S) -> WhichResult<Command>
where
	S: AsRef<OsStr>,
{
	Ok(Command::new(which(binary_name)?))
}

/// Writes text content to a new file at `path`. If the file already exists,
/// it's overwritten.
///
/// Expects are okay here because the build script panics on failure already.
fn write_to_file<P>(path: P, text: &str)
where
	P: AsRef<Path>,
{
	File::create(path)
		.expect("unable to open the file path for writing")
		.write_all(text.as_bytes())
		.expect("unable to write to the file")
}

fn get_git_commit_hash() -> String {
	let git_output = new_command("git")
		.expect("unable to locate an installed instance of Git")
		.arg("rev-parse")
		.arg("HEAD")
		.output()
		.expect("failed to execute git");
	assert!(git_output.status.success());
	String::from_utf8(git_output.stdout).expect("unable to parse the git output as valid UTF-8")
}
