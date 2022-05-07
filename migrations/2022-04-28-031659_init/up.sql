CREATE TABLE 'column_definitions' (
	'id' INTEGER PRIMARY KEY NOT NULL,
	'name' TEXT NOT NULL
);

CREATE TABLE 'locations' (
	'id' INTEGER PRIMARY KEY NOT NULL,
	'name' TEXT NOT NULL
);

CREATE TABLE 'device_key_info' (
	'id' INTEGER PRIMARY KEY NOT NULL,
	'device_id' TEXT NOT NULL UNIQUE,
	'location_id' INTEGER NOT NULL,
	'last_updated' TEXT NOT NULL,
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
