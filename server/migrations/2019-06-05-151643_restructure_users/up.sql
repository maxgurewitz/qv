DROP TABLE users;

CREATE TABLE users
(
    email VARCHAR PRIMARY KEY,
    email_verified BOOLEAN,
    name VARCHAR,
    locale VARCHAR,
    picture VARCHAR
);