ALTER TYPE usage_kind RENAME VALUE 'TimezoneCalled' TO 'TimezoneCalledUndetected';
ALTER TYPE usage_kind ADD VALUE 'TimezoneCalledDetected' AFTER 'TimezoneCalledUndetected';
ALTER TYPE usage_kind RENAME VALUE 'TimezoneSet' TO 'TimezoneSetUndetected';
ALTER TYPE usage_kind ADD VALUE 'TimezoneSetDetected' AFTER 'TimezoneSetUndetected';