//! A simple trust-based inventory-management system for tracking devices.

// Features
#![feature(min_specialization)]
#![feature(async_closure)]
// Linting Rules
#![warn(
	clippy::complexity,
	clippy::correctness,
	clippy::pedantic,
	clippy::perf,
	clippy::style,
	clippy::suspicious,
	clippy::clone_on_ref_ptr,
	clippy::dbg_macro,
	clippy::decimal_literal_representation,
	clippy::exit,
	clippy::filetype_is_file,
	clippy::if_then_some_else_none,
	clippy::non_ascii_literal,
	clippy::self_named_module_files,
	clippy::str_to_string,
	clippy::undocumented_unsafe_blocks,
	clippy::wildcard_enum_match_arm
)]
#![allow(
	clippy::cast_possible_truncation,
	clippy::cast_possible_wrap,
	clippy::cast_precision_loss,
	clippy::cast_sign_loss,
	clippy::doc_markdown,
	clippy::module_name_repetitions,
	clippy::needless_pass_by_value,
	clippy::no_effect_underscore_binding,
	clippy::similar_names,
	clippy::struct_excessive_bools,
	clippy::too_many_lines,
	clippy::unnecessary_wraps,
	clippy::wildcard_imports,
	dead_code,
	unused_macros
)]

// Macro Imports
#[macro_use]
extern crate diesel;
#[macro_use]
extern crate rocket;
#[macro_use]
extern crate serde_derive;

// Uses
use crate::routes::rocket;

// Modules
mod auth;
mod config;
mod db;
mod error;
mod id_gen;
mod routes;
mod util;

// Entry Point
#[launch]
fn launchpad() -> _ {
	rocket()
}
