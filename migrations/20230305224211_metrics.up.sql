CREATE TYPE usage_kind AS ENUM ('TimeDetect', 'TimeConvert', 'Help', 'Timezone', 'Date', 'Copy', 'CurrentTime');

CREATE TABLE usage
(
    id        BIGSERIAL PRIMARY KEY,
    timestamp TIMESTAMP(0) WITH TIME ZONE NOT NULL DEFAULT now(),
    kind      usage_kind                  NOT NULL
);

CREATE TABLE guild_count
(
    id        BIGSERIAL PRIMARY KEY,
    timestamp TIMESTAMP(0) WITH TIME ZONE NOT NULL DEFAULT now(),
    count     INTEGER                     NOT NULL
);