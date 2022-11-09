-- Remove the New Column --

ALTER TABLE column_definitions
	DROP COLUMN ordering_key;
