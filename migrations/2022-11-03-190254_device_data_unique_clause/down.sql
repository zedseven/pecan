-- Removes the unique clause from device_data.
PRAGMA foreign_keys = OFF;

CREATE TEMPORARY TABLE device_data_backup AS
SELECT *
FROM device_data;

DROP TABLE device_data;

CREATE TABLE device_data
(
	id                   INTEGER PRIMARY KEY NOT NULL,
	device_key_info_id   INTEGER             NOT NULL,
	column_definition_id INTEGER             NOT NULL,
	data_value           TEXT                NOT NULL,
	FOREIGN KEY (device_key_info_id) REFERENCES device_key_info (id),
	FOREIGN KEY (column_definition_id) REFERENCES column_definitions (id)
);

INSERT INTO device_data
	(id, device_key_info_id, column_definition_id, data_value)
SELECT id,
       device_key_info_id,
       column_definition_id,
       data_value
FROM device_data_backup;

DROP TABLE device_data_backup;

PRAGMA foreign_keys = ON;
