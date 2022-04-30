// Uses
use std::borrow::Cow;

use chrono::NaiveDateTime;

use super::schema::{column_definitions, device_data, device_key_info, locations};

// Models

#[derive(Identifiable, Queryable, Insertable, Serialize, Deserialize)]
#[diesel(table_name = column_definitions)]
#[serde(rename_all = "camelCase")]
pub struct ColumnDefinition<'a> {
	pub id: i32,
	pub name: Cow<'a, str>,
}

#[derive(Identifiable, Queryable, Insertable, Serialize, Deserialize)]
#[diesel(table_name = locations)]
#[serde(rename_all = "camelCase")]
pub struct LocationDefinition<'a> {
	pub id: i32,
	pub location: Cow<'a, str>,
}

#[derive(Identifiable, Queryable, Insertable, Serialize, Deserialize)]
#[diesel(table_name = device_key_info)]
#[serde(rename_all = "camelCase")]
pub struct DeviceKeyInfo<'a> {
	pub id: i32,
	pub device_id: Cow<'a, str>,
	pub location_id: i32,
	pub last_updated: NaiveDateTime,
}

#[derive(Associations, Identifiable, Queryable, Insertable, Serialize, Deserialize)]
#[diesel(table_name = device_data, belongs_to(DeviceInfo<'_>, foreign_key = device_key_info_id))]
#[serde(rename_all = "camelCase")]
pub struct DeviceData<'a> {
	pub id: i32,
	pub device_key_info_id: i32,
	pub column_definition_id: i32,
	pub data_value: Option<Cow<'a, str>>,
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
		locations::location,
		device_key_info::last_updated
	)
}
#[derive(Identifiable, Queryable, Serialize, Deserialize)]
#[diesel(table_name = device_key_info)]
#[serde(rename_all = "camelCase")]
pub struct DeviceInfo<'a> {
	pub id: i32,
	pub device_id: Cow<'a, str>,
	pub location: Cow<'a, str>,
	pub last_updated: NaiveDateTime,
}
