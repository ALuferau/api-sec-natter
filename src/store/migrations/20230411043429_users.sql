-- Add migration script here
CREATE TABLE users(
    user_id VARCHAR(30) PRIMARY KEY,
    pw_hash VARCHAR(255) NOT NULL
);
GRANT SELECT, INSERT ON users TO natter_api_user;
