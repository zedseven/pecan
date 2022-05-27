CREATE TABLE user_info
(
	id                     INTEGER PRIMARY KEY NOT NULL,
	source                 INTEGER             NOT NULL,
	unique_identifier      TEXT                NOT NULL UNIQUE,
	display_name           TEXT                NOT NULL,
	associated_location_id INTEGER             NULL DEFAULT NULL,
	FOREIGN KEY (associated_location_id) REFERENCES locations (id)
);

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
