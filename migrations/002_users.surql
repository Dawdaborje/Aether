-- User table definition

DEFINE TABLE users;

DEFINE FIELD name ON TABLE users TYPE string;
DEFINE FIELD email ON TABLE users TYPE string;
DEFINE FIELD hashed_password ON TABLE users TYPE string;