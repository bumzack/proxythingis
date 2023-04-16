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



INSERT INTO source (description, path_starts_with, method)
VALUES ('post a new  person', '/api', 'POST');



INSERT INTO target (description, schema, host, port, path, method, active)
VALUES ('simplest server. always returns 200 and no persons at all', 'http', 'localhost', 3040, '/api', 'POST',
        true);

INSERT INTO source2target (source_id, target_id)
VALUES (3, 5);



INSERT INTO source (description, path_starts_with, method)
VALUES ('Solr 10.0 / GET', '/solr', 'GET');
INSERT INTO source (description, path_starts_with, method)
VALUES ('Solr 10.0 / POST', '/solr', 'POST');
INSERT INTO source (description, path_starts_with, method)
VALUES ('Solr 10.0 / PUT', '/solr', 'PUT');
INSERT INTO source (description, path_starts_with, method)
VALUES ('Solr 10.0 / PATCH', '/solr', 'PATCH');

INSERT INTO target (description, schema, host, port, path, method, active)
VALUES ('Solr 10.0 / GET', 'http', 'localhost', 8984, '/', 'GET', true);
INSERT INTO target (description, schema, host, port, path, method, active)
VALUES ('Solr 10.0 / POST', 'http', 'localhost', 8984, '/', 'POST', true);
INSERT INTO target (description, schema, host, port, path, method, active)
VALUES ('Solr 10.0 / PUT', 'http', 'localhost', 8984, '/', 'PUT', true);
INSERT INTO target (description, schema, host, port, path, method, active)
VALUES ('Solr 10.0 / PATCH', 'http', 'localhost', 8984, '/', 'PATCH', true);


INSERT INTO source2target (source_id, target_id)
VALUES (4, 6);
INSERT INTO source2target (source_id, target_id)
VALUES (5, 7);
INSERT INTO source2target (source_id, target_id)
VALUES (6, 8);
INSERT INTO source2target (source_id, target_id)
VALUES (7, 9);



INSERT INTO source (description, path_starts_with, method)
VALUES ('Meilisearch 1.1.0 / GET', '/meili', 'GET');
INSERT INTO source (description, path_starts_with, method)
VALUES ('Meilisearch 1.1.0 / POST', '/meili', 'POST');
INSERT INTO source (description, path_starts_with, method)
VALUES ('Meilisearch 1.1.0 / PUT', '/meili', 'PUT');
INSERT INTO source (description, path_starts_with, method)
VALUES ('Meilisearch 1.1.0 / PATCH', '/meili', 'PATCH');

INSERT INTO target (description, schema, host, port, path, method, active)
VALUES ('Meilisearch 1.1.0 / GET', 'http', 'localhost', 18984, '/', 'GET', true);
INSERT INTO target (description, schema, host, port, path, method, active)
VALUES ('Meilisearch 1.1.0 / POST', 'http', 'localhost', 18984, '/', 'POST', true);
INSERT INTO target (description, schema, host, port, path, method, active)
VALUES ('Meilisearch 1.1.0 / PUT', 'http', 'localhost', 18984, '/', 'PUT', true);
INSERT INTO target (description, schema, host, port, path, method, active)
VALUES ('Meilisearch 1.1.0 / PATCH', 'http', 'localhost', 18984, '/', 'PATCH', true);


INSERT INTO source2target (source_id, target_id)
VALUES (8, 10);
INSERT INTO source2target (source_id, target_id)
VALUES (9, 11);
INSERT INTO source2target (source_id, target_id)
VALUES (10, 12);
INSERT INTO source2target (source_id, target_id)
VALUES (11, 13);



INSERT INTO source (description, path_starts_with, method)
VALUES ('Article Search Rust MicroServices - Solr', '/rust/solr/search', 'POST');

INSERT INTO target (description, schema, host, port, path, method, active)
VALUES ('Article Search Rust Solr / POST', 'http', 'localhost', 18600, '/api/v1/solr/article', 'POST', true);

INSERT INTO source2target (source_id, target_id)
VALUES (12, 14);



INSERT INTO source (description, path_starts_with, method)
VALUES ('Article Search Rust MicroServices - Meili', '/rust/meili/search', 'POST');

INSERT INTO target (description, schema, host, port, path, method, active)
VALUES ('Article Search Rust Meili / POST', 'http', 'localhost', 18600, '/api/v1/meili/article', 'POST', true);

-- UPDATE target SET description= 'Article Search Rust Meili / POST', path =  '/api/v1/meili/article' WHERE id = 15;
-- UPDATE target SET   path =  '/api/v1/solr/article' WHERE id = 14;


INSERT INTO source2target (source_id, target_id)
VALUES (13, 15);



INSERT INTO source (description, path_starts_with, method)
VALUES ('User Login  Rust MicroServices', '/rust/login', 'POST');

INSERT INTO target (description, schema, host, port, path, method, active)
VALUES ('Rust login / POST', 'http', 'localhost', 18982, '/api/v1/authentication/login', 'POST', true);

INSERT INTO source2target (source_id, target_id)
VALUES (14, 16);

