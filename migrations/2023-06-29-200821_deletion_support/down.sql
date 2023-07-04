-- Remove the new columns

ALTER TABLE device_key_info
	DROP COLUMN deleted;

ALTER TABLE device_components
	DROP COLUMN deleted;

ALTER TABLE device_attachments
	DROP COLUMN deleted;
