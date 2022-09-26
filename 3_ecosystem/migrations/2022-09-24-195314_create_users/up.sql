CREATE TABLE "users"
(
  id SERIAL PRIMARY KEY,
  name VARCHAR NOT NULL,
  CONSTRAINT name_unique UNIQUE( name )
);

CREATE TABLE friendship
(
  user_id INTEGER REFERENCES "users"( id ) ON DELETE CASCADE NOT NULL,
  friend_id INTEGER REFERENCES "users"( id ) ON DELETE CASCADE NOT NULL,
  PRIMARY KEY( user_id, friend_id )
);

CREATE TABLE user_password
(
  user_id INTEGER REFERENCES "users"( id ) ON DELETE CASCADE NOT NULL,
  password VARCHAR NOT NULL,
  PRIMARY KEY( user_id )
);