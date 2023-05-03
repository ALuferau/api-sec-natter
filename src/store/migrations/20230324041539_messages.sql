-- Add migration script here
CREATE TABLE messages(
    space_id BIGINT NOT NULL REFERENCES spaces(space_id),
    msg_id BIGINT PRIMARY KEY,
    author VARCHAR(30) NOT NULL,
    msg_time TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    msg_text VARCHAR(1024) NOT NULL
);
CREATE SEQUENCE msg_id_seq OWNED BY messages.msg_id;
CREATE INDEX msg_timestamp_idx ON messages(msg_time);
GRANT SELECT, INSERT ON messages TO natter_api_user;
GRANT SELECT, USAGE ON msg_id_seq TO natter_api_user;
