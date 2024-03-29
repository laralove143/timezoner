CREATE TYPE usage_kind AS ENUM ('TimeDetect', 'TimeConvert', 'Help', 'TimezoneCalled', 'TimezoneSet', 'Date', 'Copy', 'CurrentTime');

CREATE TABLE usage
(
    id        BIGSERIAL PRIMARY KEY,
    timestamp TIMESTAMP(0) WITH TIME ZONE NOT NULL DEFAULT now(),
    kind      usage_kind                  NOT NULL
);
CREATE INDEX usage_idx ON usage (kind);

CREATE TABLE guild_count
(
    id        BIGSERIAL PRIMARY KEY,
    timestamp TIMESTAMP(0) WITH TIME ZONE NOT NULL DEFAULT now(),
    count     INTEGER                     NOT NULL
);