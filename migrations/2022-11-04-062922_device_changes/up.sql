-- Adds change logging for all device-related fields.
-- There's room in the future for also supporting changes to administrative-level stuff. (new device data columns, new users, etc.)

--- Tables ---

CREATE TABLE device_changes
(
	id                 INTEGER PRIMARY KEY NOT NULL,
	device_key_info_id INTEGER             NOT NULL,
	timestamp          TIMESTAMP           NOT NULL,
	user_id            INTEGER             NULL,
	change             TEXT                NOT NULL,
	FOREIGN KEY (device_key_info_id) REFERENCES device_key_info (id),
	FOREIGN KEY (user_id) REFERENCES user_info (id),
	CHECK (json_valid(change))
);


--- Triggers ---

CREATE TRIGGER device_changes_insert_json_minification
	AFTER INSERT
	ON device_changes
BEGIN
	UPDATE device_changes
	SET change = json(change) -- Minifies the data
	WHERE id = NEW.id;
END;

CREATE TRIGGER device_changes_update_json_minification
	AFTER UPDATE
	ON device_changes
BEGIN
	UPDATE device_changes
	SET change = json(change) -- Minifies the data
	WHERE id = NEW.id;
END;


--- Data Migration ---

-- Here, there be monsters.
INSERT INTO device_changes (device_key_info_id, timestamp, user_id, change)
SELECT dki.id              AS device_key_info_id,
       dki.last_updated    AS timestamp,
       NULL                AS user_id,
       (SELECT json_group_object(name, json(value))
        FROM (SELECT 'deviceKeyInfo'                                                AS name,
                     json_object('operation', 'add', 'locationId', dki.location_id) AS value
              UNION ALL
              SELECT 'deviceData'                  AS name,
                     json_group_array(json(entry)) AS value
              FROM (SELECT json_object('columnDefinitionId', dd.column_definition_id, 'dataValue',
                                       dd.data_value) AS entry
                    FROM device_data AS dd
                    WHERE dd.device_key_info_id = dki.id)
              GROUP BY 1
              UNION ALL
              SELECT 'deviceComponents'            AS name,
                     json_group_array(json(entry)) AS value
              FROM (SELECT json_object('operation', 'add', 'componentId', dc.component_id, 'componentType',
                                       dc.component_type) AS entry
                    FROM device_components AS dc
                    WHERE dc.device_key_info_id = dki.id)
              GROUP BY 1
              UNION ALL
              SELECT 'deviceAttachments'           AS name,
                     json_group_array(json(entry)) AS value
              FROM (SELECT json_object('operation', 'add', 'attachmentId', da.attachment_id, 'description',
                                       da.description, 'fileName', da.file_name) AS entry
                    FROM device_attachments AS da
                    WHERE da.device_key_info_id = dki.id)
              GROUP BY 1)) AS change
FROM device_key_info AS dki;


--- Remove Redundant Columns ---

ALTER TABLE device_key_info
	DROP COLUMN last_updated;
