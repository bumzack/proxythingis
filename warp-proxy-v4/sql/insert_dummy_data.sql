CREATE TABLE IF NOT EXISTS source
(
    id               serial PRIMARY KEY NOT NULL,
    description      VARCHAR(200)       NOT NULL,
    method           VARCHAR(200)       NOT NULL,
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


INSERT INTO source (description, path_starts_with, method)
VALUES ('return a list of persons', '/api', 'GET');

INSERT INTO target (description, schema, host, port, path, method, active)
VALUES ('simplest server. always returns 200 and no persons at all', 'http', 'localhost', 3040, 'api/person', 'GET',
        true);


INSERT INTO target (description, schema, host, port, path, method, active)
VALUES ('Warp & Diesel (blocking)', 'http', 'localhost', 3050, '/api', 'GET', true);

INSERT INTO target (description, schema, host, port, path, method, active)
VALUES ('Warp & Tokio-PostgreSQL (async)', 'http', 'localhost', 3060, '/api', 'GET', true);


INSERT INTO source2target (source_id, target_id)
VALUES (1, 1);
INSERT INTO source2target (source_id, target_id)
VALUES (1, 2);
INSERT INTO source2target (source_id, target_id)
VALUES (1, 3);

