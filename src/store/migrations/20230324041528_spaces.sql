-- Add migration script here
CREATE TABLE spaces(
    space_id BIGINT PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    owner VARCHAR(30) NOT NULL
);
CREATE SEQUENCE space_id_seq OWNED BY spaces.space_id;
CREATE UNIQUE INDEX space_name_idx ON spaces(name);
GRANT SELECT, INSERT ON spaces TO natter_api_user;
GRANT SELECT, USAGE ON space_id_seq TO natter_api_user;
