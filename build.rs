// Uses
use std::{
	io::{stderr, stdout, Write},
	process::Command,
};

use chrono::Utc;
use which::which;

/// Builds the static site before starting up.
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

	// Get the NPM binary path
	// Previously it just used `Command::new("npm")`, but that didn't work on
	// Windows for some stupid reason
	let npm_path = which("npm").expect("unable to locate an installed instance of NPM");

	// Run the build command
	let output = Command::new(npm_path)
		.arg("run")
		.arg("build")
		.current_dir(concat!(env!("CARGO_MANIFEST_DIR"), "/web"))
		.env("BUILD_MODE", build_mode)
		.env("BUILD_VERSION", build_version)
		.env("BUILD_DATE", build_date)
		.output()
		.expect("failed to execute npm");

	println!("Svelte build: {}", output.status);
	stdout()
		.write_all(&output.stdout)
		.expect("failed to write to stdout");
	stderr()
		.write_all(&output.stderr)
		.expect("failed to write to stderr");

	// Exit if the Svelte build failed
	assert!(output.status.success());
}
