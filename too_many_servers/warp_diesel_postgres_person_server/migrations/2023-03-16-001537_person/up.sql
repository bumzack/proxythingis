CREATE TABLE IF NOT EXISTS person
(
    id
    serial
    PRIMARY
    KEY,
    firstname
    VARCHAR
(
    50
) NOT NULL,
    lastname VARCHAR
(
    50
) NOT NULL,
    created TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
                          );