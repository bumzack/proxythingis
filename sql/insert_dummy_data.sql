DROP TABLE source_stats;
DROP TABLE target_stats;

DROP TABLE source2target;
DROP TABLE source;
DROP TABLE target;

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
    port        INT                NOT NULL,
    path        VARCHAR(200)       NOT NULL,
    method      VARCHAR(200)       NOT NULL,
    active      BOOLEAN            NOT NULL,
    created     TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS source2target
(
    id        serial PRIMARY KEY NOT NULL,
    source_id INT                NOT NULL,
    target_id INT                NOT NULL,
    created   TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (source_id) REFERENCES source (id),
    FOREIGN KEY (target_id) REFERENCES target (id)
);



CREATE TABLE IF NOT EXISTS source_stats
(
    id        serial PRIMARY KEY       NOT NULL,
    hits      BIGINT                      NOT NULL,
    source_id INT                      NOT NULL,
    start     TIMESTAMP WITH TIME ZONE NOT NULL,
    stop      TIMESTAMP WITH TIME ZONE NOT NULL,
    created   TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (source_id) REFERENCES source (id)
);

CREATE TABLE IF NOT EXISTS target_stats
(
    id        serial PRIMARY KEY       NOT NULL,
    hits      BIGINT                     NOT NULL,
    min_ns    BIGINT                     NOT NULL,
    max_ns    BIGINT                     NOT NULL,
    avg_ns    BIGINT                     NOT NULL,
    start     TIMESTAMP WITH TIME ZONE NOT NULL,
    stop      TIMESTAMP WITH TIME ZONE NOT NULL,
    target_id INT                      NOT NULL,
    created   TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (target_id) REFERENCES target (id)
);

INSERT INTO source (description, path_starts_with, method)
VALUES ('return a list of persons', '/api', 'GET');


INSERT INTO target (description, schema, host, port, path, method, active)
VALUES ('simplest server. always returns 200 and no persons at all', 'http', 'localhost', 3040, '/api', 'GET',
        true);

INSERT INTO target (description, schema, host, port, path, method, active)
VALUES ('Warp & Diesel (blocking)', 'http', 'localhost', 3050, '/api', 'GET', true);

INSERT INTO target (description, schema, host, port, path, method, active)
VALUES ('Warp & Tokio-PostgreSQL (async)', 'http', 'localhost', 3060, '/api', 'GET', true);



INSERT INTO source (description, path_starts_with, method)
VALUES ('return  500 MB of data', '/data', 'GET');

INSERT INTO target (description, schema, host, port, path, method, active)
VALUES ('Big data server', 'http', 'localhost', 3070, '/data', 'GET', true);

INSERT INTO source2target (source_id, target_id)
VALUES (1, 1);
INSERT INTO source2target (source_id, target_id)
VALUES (1, 2);
INSERT INTO source2target (source_id, target_id)
VALUES (1, 3);
INSERT INTO source2target (source_id, target_id)
VALUES (2, 4);

