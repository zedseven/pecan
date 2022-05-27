--- Starting Locations ---

DELETE
FROM locations
WHERE name = 'Central Storage';


--- Column Possible Values ---

DELETE
FROM column_possible_values
WHERE column_definition_id = 1
  AND value = 'Destroyed / Disposed Of';

DELETE
FROM column_possible_values
WHERE column_definition_id = 1
  AND value = 'Not Working';

DELETE
FROM column_possible_values
WHERE column_definition_id = 1
  AND value = 'Active';


--- Column Definitions ---

DELETE
FROM column_definitions
WHERE name = 'Serial Number';

DELETE
FROM column_definitions
WHERE name = 'Model';

DELETE
FROM column_definitions
WHERE name = 'Manufacturer';

DELETE
FROM column_definitions
WHERE name = 'Status';
