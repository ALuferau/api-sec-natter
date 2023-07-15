-- Add migration script here
CREATE TABLE audit_log(
    audit_id INT NULL,
    method VARCHAR(10) NOT NULL,
    path VARCHAR(100) NOT NULL,
    user_id VARCHAR(30) NULL,
    status INT NULL,
    audit_time TIMESTAMP NOT NULL
);
CREATE SEQUENCE audit_id_seq;
GRANT SELECT, INSERT ON audit_log TO natter_api_user;
GRANT SELECT, USAGE ON audit_id_seq TO natter_api_user;
