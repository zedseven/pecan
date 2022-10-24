#![allow(deprecated)]

// Uses
use diesel::{
	backend::{self, Backend},
	deserialize::{self, FromSql},
	serialize::{self, Output, ToSql},
	sql_types::Integer,
};

// Enums

/// Represents a user source - where the user came from.
#[repr(i32)]
#[derive(Debug, Clone, Copy, FromSqlRow, AsExpression, Serialize, Deserialize)]
#[diesel(sql_type = Integer)]
pub enum UserSource {
	#[deprecated = "Currently unused, but reserved for later use."]
	Local = 1,
	Ldap = 2,
}

impl<DB> FromSql<Integer, DB> for UserSource
where
	DB: Backend,
	i32: FromSql<Integer, DB>,
{
	fn from_sql(bytes: backend::RawValue<'_, DB>) -> deserialize::Result<Self> {
		match i32::from_sql(bytes)? {
			1 => Ok(UserSource::Local),
			2 => Ok(UserSource::Ldap),
			x => Err(format!("Unrecognized variant {x}").into()),
		}
	}
}

impl<DB> ToSql<Integer, DB> for UserSource
where
	DB: Backend,
	i32: ToSql<Integer, DB>,
{
	fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, DB>) -> serialize::Result {
		match self {
			UserSource::Local => 1.to_sql(out),
			UserSource::Ldap => 2.to_sql(out),
		}
	}
}
