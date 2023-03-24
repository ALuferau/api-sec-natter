-- Add migration script here
CREATE TABLE spaces(
    space_id INT PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    owner VARCHAR(30) NOT NULL
);
CREATE SEQUENCE space_id_seq;
CREATE UNIQUE INDEX space_name_idx ON spaces(name);
