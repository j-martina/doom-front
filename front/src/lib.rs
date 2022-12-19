//! # `doom-front`
//!
//! ## About
//!
//! Comprehensive suite of frontends for domain-specific languages written for
//! Doom's source ports.
//!
//! Within this documentation, the term "lump" is used as a catch-all term for
//! a filesystem entry of some kind, whether that be a real file, a WAD archive
//! entry, or some other compressed archive entry.

pub mod cvarinfo;
pub mod loadacs;

pub type ParseError = peg::error::ParseError<<str as peg::Parse>::PositionRepr>;

use std::{hash::Hash, sync::Arc};

use indexmap::IndexSet;
use parking_lot::RwLock;
#[cfg(feature = "serde")]
use serde::Serialize;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct Span {
	start: usize,
	end: usize,
}

impl Span {
	#[must_use]
	pub fn new(start: usize, end: usize) -> Self {
		Self { start, end }
	}

	/// Verify that the span's start and end positions lie on UTF-8 character
	/// boundaries. Principally for use in a debug assertion.
	#[must_use]
	pub fn validate(&self, source: &str) -> bool {
		source.get(self.start..self.end).is_some()
	}

	#[must_use]
	#[inline(always)]
	pub fn combine(self, other: Self) -> Self {
		Self {
			start: self.start.max(other.start),
			end: self.end.max(other.end),
		}
	}

	#[must_use]
	pub fn start(&self) -> usize {
		self.start
	}

	#[must_use]
	pub fn end(&self) -> usize {
		self.end
	}
}

#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct Identifier {
	pub span: Span,
	pub string: StringHandle,
}

// String/identifier interning /////////////////////////////////////////////////

/// Points to an entry in the string interner.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct StringIndex(usize);

/// Ties a [`StringIndex`] to the interner that created it, allowing operations
/// on the contents behind the index without having to make the interner global.
#[derive(Clone)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct StringHandle {
	#[cfg_attr(feature = "serde", serde(skip))]
	interner: Arc<RwLock<Interner>>,
	index: StringIndex,
}

impl PartialEq for StringHandle {
	fn eq(&self, other: &Self) -> bool {
		Arc::ptr_eq(&self.interner, &other.interner) && self.index == other.index
	}
}

impl Eq for StringHandle {}

impl Hash for StringHandle {
	fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
		self.interner.read().get(self.index).hash(state);
	}
}

impl std::fmt::Display for StringHandle {
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		write!(f, "{:?}", self.interner.read().get(self.index))
	}
}

// Implement manually so debug printing doesn't write the interner's representation
impl std::fmt::Debug for StringHandle {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.debug_struct("StringHandle")
			.field("index", &self.index)
			.finish()
	}
}

#[derive(Debug, Default)]
pub struct Interner {
	set: IndexSet<Box<str>>,
}

impl std::fmt::Display for Interner {
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		writeln!(f, "{{")?;

		for (i, s) in self.set.iter().enumerate() {
			writeln!(f, "\t{} => {:?},", i, s)?;
		}

		write!(f, "}}")?;

		Ok(())
	}
}

impl Interner {
	#[must_use]
	#[allow(unused)]
	pub fn new_arc() -> Arc<RwLock<Self>> {
		Arc::new(RwLock::new(Self::default()))
	}

	#[must_use]
	pub fn add(&mut self, string: &str) -> StringIndex {
		StringIndex(self.set.insert_full(string.to_string().into_boxed_str()).0)
	}

	pub fn intern(this: &Arc<RwLock<Interner>>, string: &str) -> StringHandle {
		{
			let guard = this.read();

			if let Some(s) = guard.try_lookup(string) {
				return StringHandle {
					interner: this.clone(),
					index: s,
				};
			}
		}

		{
			let mut guard = this.write();

			StringHandle {
				interner: this.clone(),
				index: guard.add(string),
			}
		}
	}

	#[must_use]
	pub fn get(&self, index: StringIndex) -> &str {
		self.set[index.0].as_ref()
	}

	#[must_use]
	pub fn _lookup(&mut self, string: &str) -> StringIndex {
		if let Some(index) = self.set.get_index_of(string) {
			StringIndex(index)
		} else {
			self.add(string)
		}
	}

	#[must_use]
	pub fn try_lookup(&self, string: &str) -> Option<StringIndex> {
		self.set.get_index_of(string).map(StringIndex)
	}

	pub fn _clear(&mut self) {
		self.set.clear();
	}
}
