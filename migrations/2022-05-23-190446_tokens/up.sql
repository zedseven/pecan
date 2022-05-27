CREATE TABLE tokens
(
	id      INTEGER PRIMARY KEY NOT NULL,
	user_id INTEGER             NOT NULL,
	value   TEXT                NOT NULL,
	expires TIMESTAMP           NOT NULL,
	valid   BOOLEAN             NOT NULL DEFAULT 1,
	FOREIGN KEY (user_id) REFERENCES user_info (id)
);

-- Automatically removes invalid tokens from the table
CREATE TRIGGER remove_expired_tokens
	AFTER INSERT
	ON tokens
BEGIN
	DELETE
	FROM tokens
	WHERE strftime('%s', tokens.expires) < strftime('%s', 'now')
	   OR tokens.valid = 0;
END;
