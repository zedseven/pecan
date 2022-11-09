--- Re-Add Redundant Columns ---

PRAGMA foreign_keys = OFF;

CREATE TEMPORARY TABLE device_key_info_backup AS
SELECT *
FROM device_key_info;

DROP TABLE device_key_info;

CREATE TABLE device_key_info
(
	id           INTEGER PRIMARY KEY NOT NULL,
	device_id    TEXT                NOT NULL UNIQUE,
	location_id  INTEGER             NOT NULL,
	last_updated TIMESTAMP           NOT NULL,
	FOREIGN KEY (location_id) REFERENCES locations (id)
);

INSERT INTO device_key_info
	(id, device_id, location_id, last_updated)
SELECT dkib.id,
       dkib.device_id,
       dkib.location_id,
       (SELECT dc.timestamp
        FROM device_changes AS dc
        WHERE dc.device_key_info_id = dkib.id
        ORDER BY dc.timestamp DESC
        LIMIT 1) AS last_updated
FROM device_key_info_backup AS dkib;

DROP TABLE device_key_info_backup;

PRAGMA foreign_keys = ON;


--- Drop Triggers ---

DROP TRIGGER device_changes_update_json_minification;

DROP TRIGGER device_changes_insert_json_minification;


--- Drop Tables ---

DROP TABLE device_changes;
