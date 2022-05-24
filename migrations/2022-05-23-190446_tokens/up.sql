CREATE TABLE 'tokens'
(
	'id'      INTEGER PRIMARY KEY NOT NULL,
	'user'    TEXT                NOT NULL,
	'value'   TEXT                NOT NULL,
	'expires' TIMESTAMP           NOT NULL,
	'valid'   BOOLEAN             NOT NULL DEFAULT 1
);

-- Automatically removes bad tokens from the table
CREATE TRIGGER 'remove_expired_tokens'
	AFTER INSERT
	ON 'tokens'
BEGIN
	DELETE
	FROM 'tokens'
	WHERE strftime('%s', 'tokens'.'expires') < strftime('%s', 'now')
	   OR 'tokens'.'valid' != 1;
END;
