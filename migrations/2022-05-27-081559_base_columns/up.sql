--- Column Definitions ---

INSERT INTO column_definitions (name, not_null, unique_values, show_in_main_page, show_on_labels,
                                exclusively_possible_values)
VALUES ('Status', 1, 0, 1, 0, 1);

INSERT INTO column_definitions (name, not_null, unique_values, show_in_main_page, show_on_labels,
                                exclusively_possible_values)
VALUES ('Manufacturer', 0, 0, 1, 1, 0);

INSERT INTO column_definitions (name, not_null, unique_values, show_in_main_page, show_on_labels,
                                exclusively_possible_values)
VALUES ('Model', 1, 0, 1, 1, 0);

INSERT INTO column_definitions (name, not_null, unique_values, show_in_main_page, show_on_labels,
                                exclusively_possible_values)
VALUES ('Serial Number', 1, 1, 1, 1, 0);


--- Column Possible Values ---

-- Status
INSERT INTO column_possible_values (column_definition_id, value)
VALUES (1, 'Active');

INSERT INTO column_possible_values (column_definition_id, value)
VALUES (1, 'Not Working');

INSERT INTO column_possible_values (column_definition_id, value)
VALUES (1, 'Decommissioned');

UPDATE column_definitions
SET default_value_id = 1
WHERE id = 1;


--- Starting Locations ---

INSERT INTO locations (name)
VALUES ('Central Storage');
