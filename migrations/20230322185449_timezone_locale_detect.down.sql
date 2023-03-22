ALTER TYPE usage_kind RENAME TO usage_kind_new;
CREATE TYPE usage_kind AS ENUM (
    'TimeDetect',
    'TimeConvertByAuthor',
    'TimeConvertByNonAuthor',
    'Help',
    'TimezoneCalled',
    'TimezoneSet',
    'Date',
    'Copy',
    'CurrentTime'
    );
ALTER TABLE usage
    ALTER COLUMN kind TYPE usage_kind USING CASE
                                                WHEN kind = 'TimezoneDetect' THEN 'TimezoneCalled'
                                                ELSE kind::text::usage_kind END;
DROP TYPE usage_kind_new;