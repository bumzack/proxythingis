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
    FOREIGN KEY (source_id
        ) REFERENCES source (id),
    FOREIGN KEY (target_id) REFERENCES target (id)
);



CREATE TABLE IF NOT EXISTS source_stats
(
    id        serial PRIMARY KEY       NOT NULL,
    hits      BIGINT                   NOT NULL,
    source_id INT                      NOT NULL,
    start     TIMESTAMP WITH TIME ZONE NOT NULL,
    stop      TIMESTAMP WITH TIME ZONE NOT NULL,
    created   TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (source_id) REFERENCES source (id)
);

CREATE TABLE IF NOT EXISTS target_stats
(
    id        serial PRIMARY KEY       NOT NULL,
    hits      BIGINT                   NOT NULL,
    min_ns    BIGINT                   NOT NULL,
    max_ns    BIGINT                   NOT NULL,
    avg_ns    BIGINT                   NOT NULL,
    start     TIMESTAMP WITH TIME ZONE NOT NULL,
    stop      TIMESTAMP WITH TIME ZONE NOT NULL,
    target_id INT                      NOT NULL,
    created   TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (target_id) REFERENCES target (id)
);

INSERT INTO source (description, path_starts_with, method)
VALUES ('return a list of persons', '/api', '*');


INSERT INTO target (description, schema, host, port, path, method, active)
VALUES ('simplest server. always returns 200 and no persons at all', 'http', 'localhost', 3040, '/api', '*',
        true);

INSERT INTO target (description, schema, host, port, path, method, active)
VALUES ('Warp & Diesel (blocking)', 'http', 'localhost', 3050, '/api', '*', true);

INSERT INTO target (description, schema, host, port, path, method, active)
VALUES ('Warp & Tokio-PostgreSQL (async)', 'http', 'localhost', 3060, '/api', '*', true);



INSERT INTO source (description, path_starts_with, method)
VALUES ('return  500 MB of data', '/data', '*');

INSERT INTO target (description, schema, host, port, path, method, active)
VALUES ('Big data server', 'http', 'localhost', 3070, '/data', '*', true);

INSERT INTO source2target (source_id, target_id)
VALUES (1, 1);
INSERT INTO source2target (source_id, target_id)
VALUES (1, 2);
INSERT INTO source2target (source_id, target_id)
VALUES (1, 3);
INSERT INTO source2target (source_id, target_id)
VALUES (2, 4);



INSERT INTO source (description, path_starts_with, method)
VALUES ('person API', '/api', '*');

INSERT INTO target (description, schema, host, port, path, method, active)
VALUES ('simplest server. always returns 200 and no persons at all', 'http', 'localhost', 3040, '/api', '*',
        true);

INSERT INTO source2target (source_id, target_id)
VALUES (3, 5);



INSERT INTO source (description, path_starts_with, method)
VALUES ('Solr 10.0', '/solr', '*');

INSERT INTO target (description, schema, host, port, path, method, active)
VALUES ('Solr 10.0', 'http', 'localhost', 8984, '/', '*', true);

INSERT INTO source2target (source_id, target_id)
VALUES (4, 6);



INSERT INTO source (description, path_starts_with, method)
VALUES ('Meilisearch 1.1.0', '/meili', '*');

INSERT INTO target (description, schema, host, port, path, method, active)
VALUES ('Meilisearch 1.1.0', 'http', 'localhost', 18984, '/', '*', true);

INSERT INTO source2target (source_id, target_id)
VALUES (5, 7);


INSERT INTO source (description, path_starts_with, method)
VALUES ('Article Search Rust MicroServices - Solr', '/rust/solr/search', '*');

INSERT INTO target (description, schema, host, port, path, method, active)
VALUES ('Article Search Rust Solr v1', 'http', 'localhost', 18600, '/api/v1/solr/article', '*', true);

INSERT INTO target (description, schema, host, port, path, method, active)
VALUES ('Article Search Rust Solr v2 ', 'http', 'localhost', 18600, '/api/v2/solr/article', '*', true);

INSERT INTO source2target (source_id, target_id)
VALUES (6, 8);


INSERT INTO source2target (source_id, target_id)
VALUES (6, 9);



INSERT INTO source (description, path_starts_with, method)
VALUES ('Article Search Rust MicroServices - Meili', '/rust/meili/search', '*');

INSERT INTO target (description, schema, host, port, path, method, active)
VALUES ('Article Search Rust Meili v1', 'http', 'localhost', 18600, '/api/v1/meili/article', '*', true);

INSERT INTO target (description, schema, host, port, path, method, active)
VALUES ('Article Search Rust Meili v2', 'http', 'localhost', 18600, '/api/v2/meili/article', '*', true);


INSERT INTO source2target (source_id, target_id)
VALUES (7, 10);

INSERT INTO source2target (source_id, target_id)
VALUES (7, 11);


INSERT INTO source (description, path_starts_with, method)
VALUES ('Rust Authentication API login', '/rust/login', '*');

INSERT INTO target (description, schema, host, port, path, method, active)
VALUES ('Rust Authentication API', 'http', 'localhost', 18982, '/api/v1/authentication/login', '*', true);

INSERT INTO source2target (source_id, target_id)
VALUES (8, 12);

