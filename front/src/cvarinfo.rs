//! Parser and abstract syntax tree for ZDoom's
//! [`CVARINFO`](https://zdoom.org/wiki/CVARINFO) lump.
//!
//! Console variables or "CVars" are ZDoom's way of storing user preferences
//! and the de facto solution for persistent storage.
//!
//! The "entry point" to this module is [`CVarInfo::parse`].

use std::sync::Arc;

use parking_lot::RwLock;
#[cfg(feature = "serde")]
use serde::Serialize;

use crate::{Identifier, Interner, ParseError, Span};

/// The top of a CVARINFO abstract syntax tree.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct CVarInfo(pub Vec<CVar>);

impl CVarInfo {
	pub fn parse(input: &str, interner: &Arc<RwLock<Interner>>) -> Result<Self, ParseError> {
		parser::lump(input, interner)
	}
}

impl std::ops::Deref for CVarInfo {
	type Target = Vec<CVar>;

	fn deref(&self) -> &Self::Target {
		&self.0
	}
}

impl std::ops::DerefMut for CVarInfo {
	fn deref_mut(&mut self) -> &mut Self::Target {
		&mut self.0
	}
}

/// The AST node for a single CVar definition.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct CVar {
	pub span: Span,
	pub flags: Vec<Flag>,
	pub type_spec: TypeSpec,
	pub name: Identifier,
	pub init: Option<Initializer>,
}

/// AST node corresponding to an optional CVar qualifier.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct Flag {
	pub span: Span,
	#[cfg_attr(feature = "serde", serde(flatten))]
	pub kind: FlagKind,
}

/// The semantic component of a [`Flag`].
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub enum FlagKind {
	/// Shared between players in a network game and only mutable by the arbitrator.
	/// Persistent between saved games.
	Server,
	/// Each player has a copy that only they can mutate, and which
	/// only affects their client.
	User,
	/// Not written to save files or sent across the network.
	NoSave,
	/// This CVar isn't written to the configuration .ini file.
	NoArchive,
	/// Can only be modified if the running game allows cheating.
	Cheat,
	/// Changes to this CVar only take effect when starting a new game,
	/// unless it's been changed without using the console.
	Latch,
}

/// AST node corresponding to the CVar's type specifier.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct TypeSpec {
	pub span: Span,
	#[cfg_attr(feature = "serde", serde(flatten))]
	pub storage_type: StorageType,
}

/// The value type stored in the CVar.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub enum StorageType {
	Bool,
	Int,
	Float,
	String,
	Color,
}

/// AST node corresponding to the optional default setting for a CVar.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct Initializer {
	pub span: Span,
	#[cfg_attr(feature = "serde", serde(flatten))]
	pub value: Value,
}

/// Each variant contains the default value specified by the definition.
/// If the definition supplies no default, each type has a fallback value.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub enum Value {
	/// Defaults to `false`.
	Bool(bool),
	/// Defaults to 0.
	Int(i32),
	/// Defaults to 0.0.
	Float(f32),
	/// Defaults to "".
	String(String),
	/// Defaults to black (0, 0, 0).
	Color { red: u8, green: u8, blue: u8 },
}

peg::parser! {
	grammar parser(interner: &Arc<RwLock<Interner>>) for str {
		// Whitespace, comments ////////////////////////////////////////////////

		rule _
			= quiet! { (" " / "\n" / "\r" / "\t" / line_comment() / block_comment())* }
			/ expected!("whitespace")

		rule line_comment()
			= "//" ([^ '/' | '!'] / "//") (!"\n" [_])*
			/ "//"

		rule block_comment()
			= "/*" (!"*/" [_])* "*/"
			/ "/**/"
			/ "/***/"

		////////////////////////////////////////////////////////////////////////

		rule nocase(literal: &'static str) -> &'input str
			= input:$([_]*<{literal.len()}>)
			{?
				if input.eq_ignore_ascii_case(literal) {
					Ok(input)
				} else {
					Err(literal)
				}
			}

		rule digit_hex() -> char
			= ['a'..='f' | 'A'..='F' | '0'..='9']

		////////////////////////////////////////////////////////////////////////

		pub(super) rule lump() -> CVarInfo
			= _? definitions:(definition() ** _) _? ![_] {
				CVarInfo(definitions)
			}

		rule definition() -> CVar
			= 	start:position!()
				flags:(flag() ** _) _
				type_spec:type_spec() _
				name:name() _
				init:init()? _ ";"
				end:position!()
			{
				CVar {
					span: Span::new(start, end),
					flags,
					type_spec,
					name,
					init,
				}
			}

		rule flag() -> Flag
			=	start:position!()
				string:$(
					quiet! {
						nocase("server") / nocase("user") / nocase("nosave") /
						nocase("noarchive") / nocase("cheat") / nocase("latch")
					} /
					expected!("a scope keyword or flag qualifier")
				)
				end:position!()
			{
				let kind = if string.eq_ignore_ascii_case("server") {
					FlagKind::Server
				} else if string.eq_ignore_ascii_case("user") {
					FlagKind::User
				} else if string.eq_ignore_ascii_case("nosave") {
					FlagKind::NoSave
				} else if string.eq_ignore_ascii_case("noarchive") {
					FlagKind::NoArchive
				} else if string.eq_ignore_ascii_case("cheat") {
					FlagKind::Cheat
				} else if string.eq_ignore_ascii_case("latch") {
					FlagKind::Latch
				} else {
					unreachable!()
				};

				Flag { span: Span::new(start, end), kind, }
			}

		rule type_spec() -> TypeSpec
			=	start:position!()
				string:$(
					quiet! {
						nocase("int") /
						nocase("float") /
						nocase("color") /
						nocase("bool") /
						nocase("string")
					} /
					expected!("a type specifier")
				)
				end:position!()
			{
				let storage_type = if string.eq_ignore_ascii_case("int") {
					StorageType::Int
				} else if string.eq_ignore_ascii_case("float") {
					StorageType::Float
				} else if string.eq_ignore_ascii_case("color") {
					StorageType::Color
				} else if string.eq_ignore_ascii_case("bool") {
					StorageType::Bool
				} else if string.eq_ignore_ascii_case("string") {
					StorageType::String
				} else {
					unreachable!()
				};

				TypeSpec { span: Span::new(start, end), storage_type, }
			}

		rule name() -> Identifier
			=	start:position!()
				string:$(
					['a'..='z' | 'A'..='Z']
					['a'..='z' | 'A'..='Z' | '0'..='9' | '_']*
				)
				end:position!()
			{
				Identifier {
					span: Span::new(start, end),
					string: Interner::intern(interner, string),
				}
			}

		rule init() -> Initializer
			= 	start:position!()
				"=" _
				value:(lit_bool() / lit_float() / lit_int() / lit_color() / lit_string())
				end:position!()
			{
				Initializer { span: Span::new(start, end), value, }
			}

		rule lit_int() -> Value
			= string:dec_num_str() {?
				Ok(Value::Int(string.parse::<i32>().or(Err("32-bit integer"))?))
			}

		rule lit_float() -> Value
			= string:$(
				(dec_num_str() "." dec_num_str()) /
				(dec_num_str() ".") /
				dec_num_str()
			) {?
				Ok(
					Value::Float(
						string.parse::<f32>().or(
							Err("32-bit floating-point number")
						)?
					)
				)
			}

		rule dec_num_str() -> &'input str = $(['0'..='9']+)

		rule lit_color() -> Value
			= 	"\""
				red:$(digit_hex()*<2>) _?
				green:$(digit_hex()*<2>) _?
				blue:$(digit_hex()*<2>)
				"\""
			{?
				let red = u8::from_str_radix(red, 16).or(Err("2-digit hexadecimal string"))?;
				let green = u8::from_str_radix(green, 16).or(Err("2-digit hexadecimal string"))?;
				let blue = u8::from_str_radix(blue, 16).or(Err("2-digit hexadecimal string"))?;

				Ok(Value::Color { red, green, blue })
			}

		rule lit_bool() -> Value
			= string:$(nocase("true") / nocase("false")) {
				if string.eq_ignore_ascii_case("true") {
					Value::Bool(true)
				} else if string.eq_ignore_ascii_case("false") {
					Value::Bool(false)
				} else {
					unreachable!()
				}
			}

		rule lit_string() -> Value
			= "\"" inner:$(!("\r" [^ '\n']) [^ '\"' | '\\']+) "\"" {
				Value::String(inner.to_string())
			}
	}
}

#[cfg(test)]
mod test {
	use super::*;

	#[test]
	fn smoke() {
		const SOURCE: &str = r#"

server int delusive_bunker = 42;

USER
latch BOOL MEAT_GRINDER = false;

nOsAvE 	cheat color blueroom =
"F5 3a 95";

// a valid single-line comment

nosave server
noarchive 	float
fullConfession = 0.369;

/* ***
a valid block comment //
*/

/**/
/***/

server user nosave cheat noarchive latch server bool ch3mic4l_br3w;

user
	nosave
string
KatanaZERO
=
"LudoWic";
"#;

		let interner = Interner::new_arc();
		let cvarinfo = CVarInfo::parse(SOURCE, &interner).unwrap();

		assert_eq!(
			cvarinfo.len(),
			6,
			"Expected 6 CVar definitions, read {}.",
			cvarinfo.len()
		);

		if let Some(init) = &cvarinfo[2].init {
			assert_eq!(
				init.value,
				Value::Color {
					red: 245,
					green: 58,
					blue: 149,
				},
				"Test case [2]'s initializer is an incorrect color or a non-color."
			);
		} else {
			panic!("Test case [2]'s initializer failed to get parsed.");
		}
	}
}
