// @generated automatically by Diesel CLI.

diesel::table! {
	/// Representation of the `column_definitions` table.
	///
	/// (Automatically generated by Diesel.)
	column_definitions (id) {
		/// The `id` column of the `column_definitions` table.
		///
		/// Its SQL type is `Integer`.
		///
		/// (Automatically generated by Diesel.)
		id -> Integer,
		/// The `name` column of the `column_definitions` table.
		///
		/// Its SQL type is `Text`.
		///
		/// (Automatically generated by Diesel.)
		name -> Text,
		/// The `not_null` column of the `column_definitions` table.
		///
		/// Its SQL type is `Bool`.
		///
		/// (Automatically generated by Diesel.)
		not_null -> Bool,
		/// The `unique_values` column of the `column_definitions` table.
		///
		/// Its SQL type is `Bool`.
		///
		/// (Automatically generated by Diesel.)
		unique_values -> Bool,
		/// The `show_in_main_page` column of the `column_definitions` table.
		///
		/// Its SQL type is `Bool`.
		///
		/// (Automatically generated by Diesel.)
		show_in_main_page -> Bool,
		/// The `show_on_labels` column of the `column_definitions` table.
		///
		/// Its SQL type is `Bool`.
		///
		/// (Automatically generated by Diesel.)
		show_on_labels -> Bool,
		/// The `exclusively_possible_values` column of the `column_definitions` table.
		///
		/// Its SQL type is `Bool`.
		///
		/// (Automatically generated by Diesel.)
		exclusively_possible_values -> Bool,
		/// The `default_value_id` column of the `column_definitions` table.
		///
		/// Its SQL type is `Nullable<Integer>`.
		///
		/// (Automatically generated by Diesel.)
		default_value_id -> Nullable<Integer>,
	}
}

diesel::table! {
	/// Representation of the `column_possible_values` table.
	///
	/// (Automatically generated by Diesel.)
	column_possible_values (id) {
		/// The `id` column of the `column_possible_values` table.
		///
		/// Its SQL type is `Integer`.
		///
		/// (Automatically generated by Diesel.)
		id -> Integer,
		/// The `column_definition_id` column of the `column_possible_values` table.
		///
		/// Its SQL type is `Integer`.
		///
		/// (Automatically generated by Diesel.)
		column_definition_id -> Integer,
		/// The `value` column of the `column_possible_values` table.
		///
		/// Its SQL type is `Text`.
		///
		/// (Automatically generated by Diesel.)
		value -> Text,
	}
}

diesel::table! {
	/// Representation of the `device_components` table.
	///
	/// (Automatically generated by Diesel.)
	device_components (id) {
		/// The `id` column of the `device_components` table.
		///
		/// Its SQL type is `Integer`.
		///
		/// (Automatically generated by Diesel.)
		id -> Integer,
		/// The `device_key_info_id` column of the `device_components` table.
		///
		/// Its SQL type is `Integer`.
		///
		/// (Automatically generated by Diesel.)
		device_key_info_id -> Integer,
		/// The `component_id` column of the `device_components` table.
		///
		/// Its SQL type is `Text`.
		///
		/// (Automatically generated by Diesel.)
		component_id -> Text,
		/// The `component_type` column of the `device_components` table.
		///
		/// Its SQL type is `Text`.
		///
		/// (Automatically generated by Diesel.)
		component_type -> Text,
	}
}

diesel::table! {
	/// Representation of the `device_data` table.
	///
	/// (Automatically generated by Diesel.)
	device_data (id) {
		/// The `id` column of the `device_data` table.
		///
		/// Its SQL type is `Integer`.
		///
		/// (Automatically generated by Diesel.)
		id -> Integer,
		/// The `device_key_info_id` column of the `device_data` table.
		///
		/// Its SQL type is `Integer`.
		///
		/// (Automatically generated by Diesel.)
		device_key_info_id -> Integer,
		/// The `column_definition_id` column of the `device_data` table.
		///
		/// Its SQL type is `Integer`.
		///
		/// (Automatically generated by Diesel.)
		column_definition_id -> Integer,
		/// The `data_value` column of the `device_data` table.
		///
		/// Its SQL type is `Text`.
		///
		/// (Automatically generated by Diesel.)
		data_value -> Text,
	}
}

diesel::table! {
	/// Representation of the `device_key_info` table.
	///
	/// (Automatically generated by Diesel.)
	device_key_info (id) {
		/// The `id` column of the `device_key_info` table.
		///
		/// Its SQL type is `Integer`.
		///
		/// (Automatically generated by Diesel.)
		id -> Integer,
		/// The `device_id` column of the `device_key_info` table.
		///
		/// Its SQL type is `Text`.
		///
		/// (Automatically generated by Diesel.)
		device_id -> Text,
		/// The `location_id` column of the `device_key_info` table.
		///
		/// Its SQL type is `Integer`.
		///
		/// (Automatically generated by Diesel.)
		location_id -> Integer,
		/// The `last_updated` column of the `device_key_info` table.
		///
		/// Its SQL type is `Timestamp`.
		///
		/// (Automatically generated by Diesel.)
		last_updated -> Timestamp,
	}
}

diesel::table! {
	/// Representation of the `locations` table.
	///
	/// (Automatically generated by Diesel.)
	locations (id) {
		/// The `id` column of the `locations` table.
		///
		/// Its SQL type is `Integer`.
		///
		/// (Automatically generated by Diesel.)
		id -> Integer,
		/// The `name` column of the `locations` table.
		///
		/// Its SQL type is `Text`.
		///
		/// (Automatically generated by Diesel.)
		name -> Text,
	}
}

diesel::table! {
	/// Representation of the `tokens` table.
	///
	/// (Automatically generated by Diesel.)
	tokens (id) {
		/// The `id` column of the `tokens` table.
		///
		/// Its SQL type is `Integer`.
		///
		/// (Automatically generated by Diesel.)
		id -> Integer,
		/// The `user_id` column of the `tokens` table.
		///
		/// Its SQL type is `Integer`.
		///
		/// (Automatically generated by Diesel.)
		user_id -> Integer,
		/// The `value` column of the `tokens` table.
		///
		/// Its SQL type is `Text`.
		///
		/// (Automatically generated by Diesel.)
		value -> Text,
		/// The `expires` column of the `tokens` table.
		///
		/// Its SQL type is `Timestamp`.
		///
		/// (Automatically generated by Diesel.)
		expires -> Timestamp,
		/// The `valid` column of the `tokens` table.
		///
		/// Its SQL type is `Bool`.
		///
		/// (Automatically generated by Diesel.)
		valid -> Bool,
	}
}

diesel::table! {
	/// Representation of the `user_info` table.
	///
	/// (Automatically generated by Diesel.)
	user_info (id) {
		/// The `id` column of the `user_info` table.
		///
		/// Its SQL type is `Integer`.
		///
		/// (Automatically generated by Diesel.)
		id -> Integer,
		/// The `source` column of the `user_info` table.
		///
		/// Its SQL type is `Integer`.
		///
		/// (Automatically generated by Diesel.)
		source -> Integer,
		/// The `unique_identifier` column of the `user_info` table.
		///
		/// Its SQL type is `Text`.
		///
		/// (Automatically generated by Diesel.)
		unique_identifier -> Text,
		/// The `display_name` column of the `user_info` table.
		///
		/// Its SQL type is `Text`.
		///
		/// (Automatically generated by Diesel.)
		display_name -> Text,
		/// The `associated_location_id` column of the `user_info` table.
		///
		/// Its SQL type is `Nullable<Integer>`.
		///
		/// (Automatically generated by Diesel.)
		associated_location_id -> Nullable<Integer>,
	}
}

diesel::joinable!(column_possible_values -> column_definitions (column_definition_id));
diesel::joinable!(device_components -> device_key_info (device_key_info_id));
diesel::joinable!(device_data -> column_definitions (column_definition_id));
diesel::joinable!(device_data -> device_key_info (device_key_info_id));
diesel::joinable!(device_key_info -> locations (location_id));
diesel::joinable!(tokens -> user_info (user_id));
diesel::joinable!(user_info -> locations (associated_location_id));

diesel::allow_tables_to_appear_in_same_query!(
	column_definitions,
	column_possible_values,
	device_components,
	device_data,
	device_key_info,
	locations,
	tokens,
	user_info,
);
