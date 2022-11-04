-- Adds the requisite infrastructure for attachments.

--- Tables ---

CREATE TABLE device_attachments
(
	id                 INTEGER PRIMARY KEY NOT NULL,
	device_key_info_id INTEGER             NOT NULL,
	attachment_id      TEXT                NOT NULL,
	description        TEXT                NOT NULL,
	file_name          TEXT                NOT NULL,
	file_data          BLOB                NOT NULL,
	FOREIGN KEY (device_key_info_id) REFERENCES device_key_info (id),
	UNIQUE (device_key_info_id, attachment_id)
);
