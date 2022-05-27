--- Tables ---

CREATE TABLE column_definitions
(
	id                          INTEGER PRIMARY KEY NOT NULL,
	name                        TEXT                NOT NULL UNIQUE,
	not_null                    BOOLEAN             NOT NULL DEFAULT 0,
	unique_values               BOOLEAN             NOT NULL DEFAULT 0,
	show_in_main_page           BOOLEAN             NOT NULL DEFAULT 1,
	show_on_labels              BOOLEAN             NOT NULL DEFAULT 1,
	exclusively_possible_values BOOLEAN             NOT NULL DEFAULT 0,
	default_value_id            INTEGER             NULL     DEFAULT NULL,
	FOREIGN KEY (default_value_id) REFERENCES column_possible_values (id)
);

CREATE TABLE column_possible_values
(
	id                   INTEGER PRIMARY KEY NOT NULL,
	column_definition_id INTEGER             NOT NULL,
	value                TEXT                NOT NULL,
	FOREIGN KEY (column_definition_id) REFERENCES column_definitions (id)
);

CREATE TABLE locations
(
	id   INTEGER PRIMARY KEY NOT NULL,
	name TEXT                NOT NULL UNIQUE
);

CREATE TABLE device_key_info
(
	id           INTEGER PRIMARY KEY NOT NULL,
	device_id    TEXT                NOT NULL UNIQUE,
	location_id  INTEGER             NOT NULL,
	last_updated TIMESTAMP           NOT NULL,
	FOREIGN KEY (location_id) REFERENCES locations (id)
);

CREATE TABLE device_data
(
	id                   INTEGER PRIMARY KEY NOT NULL,
	device_key_info_id   INTEGER             NOT NULL,
	column_definition_id INTEGER             NOT NULL,
	data_value           TEXT                NOT NULL,
	FOREIGN KEY (device_key_info_id) REFERENCES device_key_info (id),
	FOREIGN KEY (column_definition_id) REFERENCES column_definitions (id)
);

CREATE TABLE device_components
(
	id                 INTEGER PRIMARY KEY NOT NULL,
	device_key_info_id INTEGER             NOT NULL,
	component_id       TEXT                NOT NULL,
	component_type     TEXT                NOT NULL,
	FOREIGN KEY (device_key_info_id) REFERENCES device_key_info (id),
	UNIQUE (device_key_info_id, component_id)
);

CREATE TABLE user_info
(
	id                     INTEGER PRIMARY KEY NOT NULL,
	source                 INTEGER             NOT NULL,
	unique_identifier      TEXT                NOT NULL UNIQUE,
	display_name           TEXT                NOT NULL,
	associated_location_id INTEGER             NULL DEFAULT NULL,
	FOREIGN KEY (associated_location_id) REFERENCES locations (id)
);

CREATE TABLE tokens
(
	id      INTEGER PRIMARY KEY NOT NULL,
	user_id INTEGER             NOT NULL,
	value   TEXT                NOT NULL UNIQUE,
	expires TIMESTAMP           NOT NULL,
	valid   BOOLEAN             NOT NULL DEFAULT 1,
	FOREIGN KEY (user_id) REFERENCES user_info (id)
);


--- Triggers ---

-- Creates the association to a location for a new user.
CREATE TRIGGER user_locations_existing
	AFTER INSERT
	ON user_info
	WHEN NEW.associated_location_id IS NULL
		AND (SELECT ll.id
		     FROM locations AS ll
		     WHERE ll.name = NEW.display_name) IS NOT NULL
BEGIN
	UPDATE user_info
	SET associated_location_id = (SELECT ll.id FROM locations AS ll WHERE ll.name = NEW.display_name)
	WHERE id = NEW.id;
END;

CREATE TRIGGER user_locations_new
	AFTER INSERT
	ON user_info
	WHEN NEW.associated_location_id IS NULL
		AND (SELECT ll.id
		     FROM locations AS ll
		     WHERE ll.name = NEW.display_name) IS NULL
BEGIN
	INSERT INTO locations (name) VALUES (NEW.display_name);

	UPDATE user_info
	SET associated_location_id = (SELECT ll.id FROM locations AS ll WHERE ll.name = NEW.display_name)
	WHERE id = NEW.id;
END;

-- Cleans up any existing tokens for a user when they're removed from the system.
CREATE TRIGGER user_deleted
	AFTER DELETE
	ON user_info
	FOR EACH ROW
BEGIN
	DELETE FROM tokens WHERE user_id = OLD.id;
END;

-- Automatically removes invalid tokens from the table.
CREATE TRIGGER remove_expired_tokens
	AFTER INSERT
	ON tokens
BEGIN
	DELETE
	FROM tokens
	WHERE strftime('%s', tokens.expires) < strftime('%s', 'now')
	   OR tokens.valid = 0;
END;
