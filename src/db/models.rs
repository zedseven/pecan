// Uses
use std::borrow::Cow;

use chrono::NaiveDateTime;
use diesel::sql_types::{Integer, Text, Timestamp};

use super::schema::*;

// Models

#[derive(Identifiable, Queryable, Serialize, Deserialize, Debug)]
#[diesel(table_name = column_definitions)]
#[serde(rename_all = "camelCase")]
pub struct ColumnDefinition<'a> {
	pub id: i32,
	pub name: Cow<'a, str>,
}
#[derive(Insertable, Debug)]
#[diesel(table_name = column_definitions)]
pub struct ColumnDefinitionNew<'a> {
	pub name: Cow<'a, str>,
}

#[derive(Identifiable, Queryable, Serialize, Deserialize, Debug)]
#[diesel(table_name = locations)]
#[serde(rename_all = "camelCase")]
pub struct LocationDefinition<'a> {
	pub id: i32,
	pub name: Cow<'a, str>,
}
#[derive(Insertable, Debug)]
#[diesel(table_name = locations)]
pub struct LocationDefinitionNew<'a> {
	pub name: Cow<'a, str>,
}

#[derive(Identifiable, Queryable, Serialize, Deserialize, Debug)]
#[diesel(table_name = device_key_info)]
#[serde(rename_all = "camelCase")]
pub struct DeviceKeyInfo<'a> {
	pub id: i32,
	pub device_id: Cow<'a, str>,
	pub location_id: i32,
	pub last_updated: NaiveDateTime,
}
#[derive(Insertable, Debug)]
#[diesel(table_name = device_key_info)]
pub struct DeviceKeyInfoNew<'a> {
	pub device_id: Cow<'a, str>,
	pub location_id: i32,
	pub last_updated: NaiveDateTime,
}

#[derive(Associations, Identifiable, Queryable, Serialize, Deserialize, Debug)]
#[diesel(table_name = device_data, belongs_to(DeviceInfo<'_>, foreign_key = device_key_info_id))]
#[serde(rename_all = "camelCase")]
pub struct DeviceData<'a> {
	pub id: i32,
	pub device_key_info_id: i32,
	pub column_definition_id: i32,
	pub data_value: Cow<'a, str>,
}
#[derive(Insertable, Debug)]
#[diesel(table_name = device_data)]
pub struct DeviceDataNew<'a> {
	pub device_key_info_id: i32,
	pub column_definition_id: i32,
	pub data_value: Cow<'a, str>,
}

// Select Definitions

macro_rules! select_def {
	($select_name:ident : $type_label:ident = $def:tt) => {
		pub type $type_label = $def;
		pub const $select_name: $type_label = $def;
	};
}

select_def! {
	DEVICE_INFO: DeviceInfoSelect = (
		device_key_info::id,
		device_key_info::device_id,
		locations::id,
		locations::name,
		device_key_info::last_updated
	)
}
#[derive(Identifiable, Queryable, Serialize, Deserialize, Debug)]
#[diesel(table_name = device_key_info)]
#[serde(rename_all = "camelCase")]
pub struct DeviceInfo<'a> {
	pub id: i32,
	pub device_id: Cow<'a, str>,
	pub location_id: i32,
	pub location: Cow<'a, str>,
	pub last_updated: NaiveDateTime,
}
// This is ugly, but from what I can tell, required. Raw SQL queries' results
// are deserialised by name, not position, so they require this separate trait
// that doesn't seem to be compatible with the rest.
#[derive(QueryableByName, Debug)]
pub struct DeviceInfoByName<'a> {
	#[diesel(sql_type = Integer)]
	pub id: i32,
	#[diesel(sql_type = Text)]
	pub device_id: Cow<'a, str>,
	#[diesel(sql_type = Integer)]
	pub location_id: i32,
	#[diesel(sql_type = Text)]
	pub location: Cow<'a, str>,
	#[diesel(sql_type = Timestamp)]
	pub last_updated: NaiveDateTime,
}
impl<'a> From<DeviceInfoByName<'a>> for DeviceInfo<'a> {
	fn from(by_name: DeviceInfoByName<'a>) -> Self {
		Self {
			id: by_name.id,
			device_id: by_name.device_id,
			location_id: by_name.location_id,
			location: by_name.location,
			last_updated: by_name.last_updated,
		}
	}
}
