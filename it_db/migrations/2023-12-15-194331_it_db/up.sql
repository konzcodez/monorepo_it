CREATE TABLE computers (
  id serial PRIMARY KEY,
  ip VARCHAR ( 12 ) NOT NULL,
  name VARCHAR ( 25 ) NOT NULL,
  os VARCHAR ( 5 ) NOT NULL,
  snum VARCHAR ( 25 ) NOT NULL,
  notes TEXT NOT NULL,
  model VARCHAR ( 25 ) NOT NULL,
  manufacturer VARCHAR ( 25 ) NOT NULL,
  cpu VARCHAR ( 25 ) NOT NULL,
  memory VARCHAR ( 25 ) NOT NULL,
  storage VARCHAR ( 25 ) NOT NULL,
  installdate VARCHAR ( 25 ) NOT NULL
);
