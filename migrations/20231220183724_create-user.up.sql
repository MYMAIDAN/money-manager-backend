-- Add up migration script here
CREATE TABLE users (
  id UUID PRIMARY KEY,
  name VARCHAR NOT NULL,
  surname TEXT NOT NULL,
  email VARCHAR NOT NULL,
  password VARCHAR NOT NULL

)