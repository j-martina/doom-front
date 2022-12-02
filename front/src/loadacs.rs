//! A struct representing the [`LOADACS`](https://zdoom.org/wiki/LOADACS) lump.
//!
//! Parsing these is as simple as breaking up an input string into
//! whitespace-separated ASCII words.

/// Represents a [`LOADACS`](https://zdoom.org/wiki/LOADACS) lump; contains
/// a list of bytecode object file names for the engine to load.
pub struct LoadAcs {
	pub objects: Vec<String>,
}

impl LoadAcs {
	pub fn parse(string: &str) -> Self {
		Self {
			objects: string
				.split(char::is_whitespace)
				.map(|s| s.to_string())
				.collect(),
		}
	}
}
