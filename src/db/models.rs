#![allow(clippy::extra_unused_lifetimes)]

// Uses
use std::borrow::Cow;

use chrono::NaiveDateTime;
use diesel::{
	dsl::Nullable,
	sql_types::{Integer, Text, Timestamp},
	NullableExpressionMethods,
};

use super::{enums::UserSource, schema::*};

// Models

#[derive(Identifiable, Queryable, Serialize, Deserialize, Debug)]
#[diesel(table_name = user_info)]
#[serde(rename_all = "camelCase")]
pub struct User {
	pub id: i32,
	pub source: UserSource,
	pub unique_identifier: String, // Uses owned data because it needs to be passed around
	pub display_name: String,
	pub associated_location_id: Option<i32>,
}
#[derive(Insertable, Debug)]
#[diesel(table_name = user_info)]
pub struct UserNew {
	pub source: UserSource,
	pub unique_identifier: String,
	pub display_name: String,
}
#[derive(Associations, Identifiable, Queryable, Serialize, Deserialize, Debug)]
#[diesel(table_name = tokens, belongs_to(User, foreign_key = user_id))]
#[serde(rename_all = "camelCase")]
pub struct Token<'a> {
	pub id: i32,
	pub user_id: i32,
	pub value: Cow<'a, str>,
	pub expires: NaiveDateTime,
	pub valid: bool,
}
#[derive(Insertable, Debug)]
#[diesel(table_name = tokens)]
pub struct TokenNew<'a> {
	pub user_id: i32,
	pub value: Cow<'a, str>,
	pub expires: NaiveDateTime,
}
#[derive(Identifiable, Queryable, Serialize, Deserialize, Debug)]
#[diesel(table_name = column_definitions)]
#[serde(rename_all = "camelCase")]
pub struct ColumnDefinition<'a> {
	pub id: i32,
	pub name: Cow<'a, str>,
	pub not_null: bool,
	pub unique_values: bool,
	pub show_in_main_page: bool,
	pub show_on_labels: bool,
	pub exclusively_possible_values: bool,
	pub default_value_id: Option<i32>,
}
#[derive(Insertable, Debug)]
#[diesel(table_name = column_definitions)]
pub struct ColumnDefinitionNew<'a> {
	pub name: Cow<'a, str>,
	pub not_null: bool,
	pub unique_values: bool,
	pub show_in_main_page: bool,
	pub show_on_labels: bool,
	pub exclusively_possible_values: bool,
	pub default_value_id: Option<i32>,
}
#[derive(Associations, Identifiable, Queryable, Serialize, Deserialize, Debug)]
#[diesel(table_name = column_possible_values, belongs_to(ColumnDefinitionSelected<'_>, foreign_key = column_definition_id))]
#[serde(rename_all = "camelCase")]
pub struct ColumnPossibleValue<'a> {
	pub id: i32,
	pub column_definition_id: i32,
	pub value: Cow<'a, str>,
}
#[derive(Insertable, Debug)]
#[diesel(table_name = column_possible_values)]
pub struct ColumnPossibleValueNew<'a> {
	pub column_definition_id: i32,
	pub value: Cow<'a, str>,
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

#[derive(Associations, Identifiable, Queryable, Serialize, Deserialize, Debug)]
#[diesel(table_name = device_components, belongs_to(DeviceInfo<'_>, foreign_key = device_key_info_id))]
#[serde(rename_all = "camelCase")]
pub struct DeviceComponent<'a> {
	pub id: i32,
	pub device_key_info_id: i32,
	pub component_id: Cow<'a, str>,
	pub component_type: Cow<'a, str>,
}
#[derive(Insertable, Debug)]
#[diesel(table_name = device_components)]
pub struct DeviceComponentNew<'a> {
	pub device_key_info_id: i32,
	pub component_id: Cow<'a, str>,
	pub component_type: Cow<'a, str>,
}

// Select Definitions

macro_rules! select_def {
	($select_name:ident : $type_label:ident = $def:tt) => {
		pub type $type_label = $def;
		pub const $select_name: $type_label = $def;
	};
}

select_def! {
	USER: UserSelect = (
		user_info::id,
		user_info::source,
		user_info::unique_identifier,
		user_info::display_name,
		user_info::associated_location_id,
	)
}

select_def! {
	DEVICE_INFO: DeviceInfoSelect = (
		device_key_info::id,
		device_key_info::device_id,
		device_key_info::location_id,
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

// Can't use `select_def` because of the nullable columns.
pub type ColumnDefinitionSelect = (
	column_definitions::id,
	column_definitions::name,
	column_definitions::not_null,
	column_definitions::unique_values,
	column_definitions::show_in_main_page,
	column_definitions::show_on_labels,
	column_definitions::exclusively_possible_values,
	column_definitions::default_value_id,
	Nullable<column_possible_values::value>,
);
pub const COLUMN_DEFINITION: &'static dyn Fn() -> ColumnDefinitionSelect = &|| {
	(
		column_definitions::id,
		column_definitions::name,
		column_definitions::not_null,
		column_definitions::unique_values,
		column_definitions::show_in_main_page,
		column_definitions::show_on_labels,
		column_definitions::exclusively_possible_values,
		column_definitions::default_value_id,
		column_possible_values::value.nullable(),
	)
};
#[derive(Identifiable, Queryable, Serialize, Deserialize, Debug)]
#[diesel(table_name = column_definitions)]
#[serde(rename_all = "camelCase")]
pub struct ColumnDefinitionSelected<'a> {
	pub id: i32,
	pub name: Cow<'a, str>,
	pub not_null: bool,
	pub unique_values: bool,
	pub show_in_main_page: bool,
	pub show_on_labels: bool,
	pub exclusively_possible_values: bool,
	pub default_value_id: Option<i32>,
	pub default_value: Option<Cow<'a, str>>,
}
