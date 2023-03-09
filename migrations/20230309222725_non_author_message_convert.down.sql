ALTER TYPE usage_kind RENAME TO usage_kind_new;
CREATE TYPE usage_kind AS ENUM ('TimeDetect', 'TimeConvert', 'Help', 'TimezoneCalled', 'TimezoneSet', 'Date', 'Copy', 'CurrentTime');
ALTER TABLE usage
    ALTER COLUMN kind TYPE usage_kind USING CASE
                                                WHEN kind = 'TimeConvertByAuthor' THEN 'TimeConvert'
                                                WHEN kind = 'TimeConvertByNonAuthor' THEN 'TimeConvert'
                                                ELSE kind::text::usage_kind END;
DROP TYPE usage_kind_new;