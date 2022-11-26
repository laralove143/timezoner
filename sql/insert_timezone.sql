INSERT INTO timezones
VALUES ($1, $2) ON CONFLICT (user_id) DO
UPDATE
SET timezone = $2;