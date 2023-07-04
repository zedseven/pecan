-- Adds tracking for whether devices and device components are deleted.

-- device_key_info

CREATE TEMPORARY TABLE device_key_info_backup AS
SELECT *
FROM device_key_info;

DROP TABLE device_key_info;

CREATE TABLE device_key_info
(
	id          INTEGER PRIMARY KEY NOT NULL,
	device_id   TEXT                NOT NULL UNIQUE,
	deleted     BOOLEAN             NOT NULL DEFAULT 0,
	location_id INTEGER             NOT NULL,
	FOREIGN KEY (location_id) REFERENCES locations (id)
);

INSERT INTO device_key_info
	(id, device_id, deleted, location_id)
SELECT id,
       device_id,
       0 as deleted,
       location_id
FROM device_key_info_backup;

DROP TABLE device_key_info_backup;

-- device_components

CREATE TEMPORARY TABLE device_components_backup AS
SELECT *
FROM device_components;

DROP TABLE device_components;

CREATE TABLE device_components
(
	id                 INTEGER PRIMARY KEY NOT NULL,
	device_key_info_id INTEGER             NOT NULL,
	component_id       TEXT                NOT NULL,
	deleted            BOOLEAN             NOT NULL DEFAULT 0,
	component_type     TEXT                NOT NULL,
	FOREIGN KEY (device_key_info_id) REFERENCES device_key_info (id),
	UNIQUE (device_key_info_id, component_id)
);

INSERT INTO device_components
	(id, device_key_info_id, component_id, deleted, component_type)
SELECT id,
       device_key_info_id,
       component_id,
       0 as deleted,
       component_type
FROM device_components_backup;

DROP TABLE device_components_backup;

-- device_attachments

CREATE TEMPORARY TABLE device_attachments_backup AS
SELECT *
FROM device_attachments;

DROP TABLE device_attachments;

CREATE TABLE device_attachments
(
	id                 INTEGER PRIMARY KEY NOT NULL,
	device_key_info_id INTEGER             NOT NULL,
	attachment_id      TEXT                NOT NULL,
	deleted            BOOLEAN             NOT NULL DEFAULT 0,
	description        TEXT                NOT NULL,
	file_name          TEXT                NOT NULL,
	file_data          BLOB                NOT NULL,
	FOREIGN KEY (device_key_info_id) REFERENCES device_key_info (id),
	UNIQUE (device_key_info_id, attachment_id)
);

INSERT INTO device_attachments
(id, device_key_info_id, attachment_id, deleted, description, file_name, file_data)
SELECT id,
       device_key_info_id,
       attachment_id,
       0 as deleted,
       description,
       file_name,
       file_data
FROM device_attachments_backup;

DROP TABLE device_attachments_backup;
