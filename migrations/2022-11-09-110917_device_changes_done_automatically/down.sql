-- Remove the New Column --

ALTER TABLE device_changes
	DROP COLUMN done_automatically;
