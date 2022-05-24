//! The module for handling generic ID generation.
//!
//! Implementations for specific uses are elsewhere.

// Uses
use rand::{distributions::Distribution, thread_rng, Rng};

use crate::error::Error;

/// A completely generic ID generator that generates unused IDs from whatever
/// `distribution` is provided, as long as the `distribution` provides values
/// that can be converted to `char` values infallibly.
///
/// It verifies that the new ID is unused by executing the predicate
/// `should_use`. This can query a database or whatever else.
pub fn gen_new_id<D, I, P>(
	distribution: D,
	length: usize,
	mut should_use: P,
) -> Result<String, Error>
where
	D: Distribution<I> + Copy,
	char: From<I>,
	P: FnMut(&str) -> Result<bool, Error>,
{
	let mut rng = thread_rng();

	let mut new_id = String::with_capacity(length);
	loop {
		// Generate the new ID
		new_id.clear();
		(&mut rng)
			.sample_iter(distribution)
			.take(length)
			.map(char::from)
			.for_each(|c| new_id.push(c));

		// Verify that the ID can be used
		if !should_use(new_id.as_str())? {
			break;
		}
	}

	Ok(new_id)
}

// Distributions

/// Provides ASCII numbers 0-9.
///
/// Largely copied from [`Alphanumeric`].
///
/// [`Alphanumeric`]: rand::distributions::Alphanumeric
#[derive(Debug, Copy, Clone)]
pub struct NumericAscii;

impl NumericAscii {
	pub const RANGE: u32 = 10;
}

impl Distribution<u8> for NumericAscii {
	fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> u8 {
		const GEN_CHARSET: &[u8] = b"0123456789";
		loop {
			let var = rng.next_u32() >> (32 - 4); // 4 bits needed to represent values up to 10 (2^4 = 16)
			if var < Self::RANGE {
				return GEN_CHARSET[var as usize];
			}
		}
	}
}

/// Provides ASCII Base64 characters from the
/// [Bcrypt](https://docs.rs/base64/latest/base64/enum.CharacterSet.html#variant.Bcrypt)
/// set.
///
/// Largely copied from [`Alphanumeric`].
///
/// [`Alphanumeric`]: rand::distributions::Alphanumeric
#[derive(Debug, Copy, Clone)]
pub struct Base64;

impl Base64 {
	pub const RANGE: u32 = 2 + 26 + 26 + 10;
}

impl Distribution<u8> for Base64 {
	fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> u8 {
		const GEN_CHARSET: &[u8] = b"./\
				ABCDEFGHIJKLMNOPQRSTUVWXYZ\
                abcdefghijklmnopqrstuvwxyz\
                0123456789";
		// 6 bits needed to represent values up to 64 (2^6 = 64)
		GEN_CHARSET[(rng.next_u32() >> (32 - 6)) as usize]
	}
}
