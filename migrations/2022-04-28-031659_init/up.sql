CREATE TABLE 'column_definitions' (
	'id' INTEGER PRIMARY KEY NOT NULL,
	'name' TEXT NOT NULL,
	'not_null' BOOLEAN NOT NULL DEFAULT 0,
	'unique_values' BOOLEAN NOT NULL DEFAULT 0,
	'show_in_main_page' BOOLEAN NOT NULL DEFAULT 1,
	'exclusively_possible_values' BOOLEAN NOT NULL DEFAULT 0
);

CREATE TABLE 'column_possible_values' (
	'id' INTEGER PRIMARY KEY NOT NULL,
	'column_definition_id' INTEGER NOT NULL,
	'value' TEXT NOT NULL,
	FOREIGN KEY ('column_definition_id') REFERENCES 'column_definitions'('id')
);

CREATE TABLE 'locations' (
	'id' INTEGER PRIMARY KEY NOT NULL,
	'name' TEXT NOT NULL
);

CREATE TABLE 'device_key_info' (
	'id' INTEGER PRIMARY KEY NOT NULL,
	'device_id' TEXT NOT NULL UNIQUE,
	'location_id' INTEGER NOT NULL,
	'last_updated' TIMESTAMP NOT NULL,
	FOREIGN KEY ('location_id') REFERENCES 'locations'('id')
);

CREATE TABLE 'device_data' (
	'id' INTEGER PRIMARY KEY NOT NULL,
	'device_key_info_id' INTEGER NOT NULL,
	'column_definition_id' INTEGER NOT NULL,
	'data_value' TEXT NOT NULL,
	FOREIGN KEY ('device_key_info_id') REFERENCES 'device_key_info'('id'),
	FOREIGN KEY ('column_definition_id') REFERENCES 'column_definitions'('id')
);

CREATE TABLE 'device_components' (
    'id' INTEGER PRIMARY KEY NOT NULL,
    'device_key_info_id' INTEGER NOT NULL,
	'component_id' TEXT NOT NULL,
	'component_type' TEXT NOT NULL,
	FOREIGN KEY ('device_key_info_id') REFERENCES 'device_key_info'('id'),
	UNIQUE ('device_key_info_id', 'component_id')
);
