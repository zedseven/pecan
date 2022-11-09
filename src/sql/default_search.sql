-- Performs a search for devices using the default values for each column.

-- @formatter:off
SELECT
	dki.id,
	dki.device_id,
	dki.location_id,
	l.name AS location,
	(
		SELECT
			dc.timestamp
		FROM device_changes AS dc
		WHERE
		    dc.device_key_info_id = dki.id AND
		    dc.done_automatically = 0
		ORDER BY dc.timestamp DESC
		LIMIT 1
	) AS last_updated
FROM device_key_info AS dki
INNER JOIN locations AS l
    ON l.id = dki.location_id
WHERE
(
    SELECT
        COUNT(cd.id)
    FROM column_definitions AS cd
    WHERE cd.default_value_id IS NOT NULL
        AND cd.show_in_main_page != 0
) = (
	SELECT
	    COUNT(cd.id)
	FROM column_definitions AS cd
	LEFT JOIN device_data AS dd
	    ON dd.column_definition_id = cd.id
	LEFT JOIN column_possible_values AS cpv
	    ON cpv.id = cd.default_value_id AND cpv.column_definition_id = cd.id
	WHERE dd.device_key_info_id = dki.id
		AND dd.id IS NOT NULL
	    AND cd.show_in_main_page != 0
		AND dd.data_value LIKE cpv.value
)
ORDER BY last_updated DESC
