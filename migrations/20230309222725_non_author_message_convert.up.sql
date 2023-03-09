ALTER TYPE usage_kind RENAME VALUE 'TimeConvert' TO 'TimeConvertByAuthor';
ALTER TYPE usage_kind ADD VALUE 'TimeConvertByNonAuthor' AFTER 'TimeConvertByAuthor';