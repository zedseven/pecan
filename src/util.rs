// Uses
use rand::{
	distributions::{Distribution, Uniform},
	thread_rng,
};

/// Generates a new device ID.
///
/// This is based on the [Microsoft Alphabet] - the set of characters they use
/// to generate product keys, designed to minimise ambiguity and prevent words
/// from being spelled by accident.
///
/// [Microsoft Alphabet]: https://www.techtalkz.com/threads/alphabet-letters-not-used-in-microsoft-product-keys.82675/#post-349424
pub fn gen_new_id() -> String {
	const PREFIX: &str = "T-"; // TODO: Make this configurable
	const LENGTH: usize = 8;
	const CHAR_SET: &[char] = &[
		'2', '3', '4', '6', '7', '9', 'C', 'D', 'F', 'G', 'H', 'J', 'K', 'M', 'P', 'Q', 'R', 'T',
		'V', 'W', 'X', 'Y',
	];

	let mut result = String::with_capacity(LENGTH + PREFIX.len());
	let uniform = Uniform::from(0..CHAR_SET.len());
	let mut rng = thread_rng();
	result.push_str(PREFIX);
	for _ in 0..LENGTH {
		result.push(CHAR_SET[uniform.sample(&mut rng)]);
	}

	result
}
