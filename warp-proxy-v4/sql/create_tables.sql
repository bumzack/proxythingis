CREATE TABLE IF NOT EXISTS source
(
    id               serial PRIMARY KEY NOT NULL,
    description      VARCHAR(200)       NOT NULL,
    path_starts_with VARCHAR(200)       NOT NULL,
    created          TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS target
(
    id          serial PRIMARY KEY NOT NULL,
    description VARCHAR(200)       NOT NULL,
    "schema"    VARCHAR(200)       NOT NULL,
    host        VARCHAR(200)       NOT NULL,
    port        INTEGER            NOT NULL,
    path        VARCHAR(200)       NOT NULL,
    method      VARCHAR(200)       NOT NULL,
    active      boolean            NOT NULL,
    created     TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS source2target
(
    id        serial PRIMARY KEY NOT NULL,
    source_id int                NOT NULL,
    target_id int                NOT NULL,
    created   TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (source_id) REFERENCES source (id),
    FOREIGN KEY (target_id) REFERENCES target (id)
);
