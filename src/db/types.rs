// Uses
use diesel::{
	backend::{Backend, RawValue},
	deserialize::{self, FromSql},
	serialize::{self, Output, ToSql},
	sql_types::Integer,
};
use serde_repr::{Deserialize_repr, Serialize_repr};

/// Represents the setting for a column definition about how the possible values
/// should be handled.
#[repr(i32)]
#[derive(Debug, Clone, Copy, FromSqlRow, AsExpression, Serialize_repr, Deserialize_repr)]
#[diesel(sql_type = Integer)]
pub enum PossibleValuesSetting {
	/// Free-typed values don't use the `column_possible_values` table, and
	/// allow the user to type whatever they want.
	///
	/// The flexibility can be good, but it allows for typos and multiple
	/// variants of the same thing. (decreasing searchability)
	FreeTyped = 1,
	/// Suggested values show up as the user types, but don't force the user to
	/// pick them. They're merely suggestions to prevent manual typing unless
	/// necessary.
	Suggested = 2,
	/// Restricted values force the user to choose from a pre-made list.
	///
	/// This prevents typos and multiple variants altogether, provided the
	/// initial list is good.
	Restricted = 3,
}

impl<DB> FromSql<Integer, DB> for PossibleValuesSetting
where
	DB: Backend,
	i32: FromSql<Integer, DB>,
{
	fn from_sql(bytes: RawValue<DB>) -> deserialize::Result<Self> {
		match i32::from_sql(bytes)? {
			1 => Ok(PossibleValuesSetting::FreeTyped),
			2 => Ok(PossibleValuesSetting::Suggested),
			3 => Ok(PossibleValuesSetting::Restricted),
			x => Err(format!("Unrecognized variant {}", x).into()),
		}
	}
}

impl<DB> ToSql<Integer, DB> for PossibleValuesSetting
where
	DB: Backend,
	i32: ToSql<Integer, DB>,
{
	fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, DB>) -> serialize::Result {
		match self {
			PossibleValuesSetting::FreeTyped => 1.to_sql(out),
			PossibleValuesSetting::Suggested => 2.to_sql(out),
			PossibleValuesSetting::Restricted => 3.to_sql(out),
		}
	}
}
