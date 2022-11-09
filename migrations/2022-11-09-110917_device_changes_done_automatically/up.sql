-- Adds a marker that indicates the change was made automatically (and therefore shouldn't be used to determine last_updated)

CREATE TEMPORARY TABLE device_changes_backup AS
SELECT *
FROM device_changes;

DROP TABLE device_changes;

CREATE TABLE device_changes
(
	id                 INTEGER PRIMARY KEY NOT NULL,
	device_key_info_id INTEGER             NOT NULL,
	timestamp          TIMESTAMP           NOT NULL,
	done_automatically BOOLEAN             NOT NULL DEFAULT 0,
	user_id            INTEGER             NULL,
	change             TEXT                NOT NULL,
	FOREIGN KEY (device_key_info_id) REFERENCES device_key_info (id),
	FOREIGN KEY (user_id) REFERENCES user_info (id),
	CHECK (json_valid(change))
);

INSERT INTO device_changes
(id, device_key_info_id, timestamp, done_automatically, user_id, change)
SELECT id,
       device_key_info_id,
       timestamp,
       -- The aggregate changes logged in the 2022-11-04-062922_device_changes migration *do not* count as automatic
       -- changes, since they represent changes made by users
       0 as done_automatically,
       user_id,
       change
FROM device_changes_backup;

DROP TABLE device_changes_backup;
