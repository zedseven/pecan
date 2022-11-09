-- Adds customisable column ordering for column_definitions.

CREATE TEMPORARY TABLE column_definitions_backup AS
SELECT *
FROM column_definitions;

DROP TABLE column_definitions;

CREATE TABLE column_definitions
(
	id                          INTEGER PRIMARY KEY NOT NULL,
	name                        TEXT                NOT NULL UNIQUE,
	ordering_key                INTEGER             NULL     DEFAULT NULL,
	not_null                    BOOLEAN             NOT NULL DEFAULT 0,
	unique_values               BOOLEAN             NOT NULL DEFAULT 0,
	show_in_main_page           BOOLEAN             NOT NULL DEFAULT 1,
	show_on_labels              BOOLEAN             NOT NULL DEFAULT 1,
	exclusively_possible_values BOOLEAN             NOT NULL DEFAULT 0,
	default_value_id            INTEGER             NULL     DEFAULT NULL,
	FOREIGN KEY (default_value_id) REFERENCES column_possible_values (id)
);

INSERT INTO column_definitions
(id, name, ordering_key, not_null, unique_values, show_in_main_page, show_on_labels, exclusively_possible_values,
 default_value_id)
SELECT id,
       name,
       id AS ordering_key, -- Default to the column IDs for ordering
       not_null,
       unique_values,
       show_in_main_page,
       show_on_labels,
       exclusively_possible_values,
       default_value_id
FROM column_definitions_backup;

DROP TABLE column_definitions_backup;
