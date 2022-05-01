// Uses
use std::{
	io::{stderr, stdout, Write},
	process::Command,
};

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
		"/web"
	));

	// Set the build mode for Svelte based on the Rust build mode
	let build_mode = if cfg!(debug_assertions) {
		"development"
	} else {
		"production"
	};

	// Run the build command
	let output = Command::new("npm")
		.arg("run")
		.arg("build")
		.current_dir(concat!(env!("CARGO_MANIFEST_DIR"), "/web"))
		.env("BUILD_MODE", build_mode)
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
