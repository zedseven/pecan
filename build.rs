// Uses
use std::process::Command;

/// Builds the static site before starting up.
fn main() {
	// Tell Cargo to rebuild Svelte if anything changed in its source directory
	println!("cargo:rerun-if-changed=web/src");

	// Set the build mode for Svelte based on the Rust build mode
	let build_mode = if cfg!(debug_assertions) {
		"development"
	} else {
		"production"
	};

	// Run the build command
	Command::new("npm")
		.arg("run")
		.arg("build")
		.current_dir("web")
		.env("BUILD_MODE", build_mode)
		.output()
		.expect("unable to build the static site");
}
