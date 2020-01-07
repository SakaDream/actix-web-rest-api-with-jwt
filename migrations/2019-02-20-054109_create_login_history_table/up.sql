-- Your SQL goes here
CREATE TABLE login_history
(
    id SERIAL PRIMARY KEY NOT NULL,
    user_id BIGINT NOT NULL REFERENCES users(id),
    login_timestamp TIMESTAMP WITH TIME ZONE NOT NULL
);
