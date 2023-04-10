-- Add migration script here

CREATE TABLE IF NOT EXISTS models (
  id INT PRIMARY KEY,
  identifier VARCHAR(255),
  name VARCHAR(255) UNIQUE
);

CREATE TABLE IF NOT EXISTS forecasts (
  id serial PRIMARY KEY,
  id_spot INT NOT NULL,
  id_model INT NOT NULL,
  init_date TIMESTAMP NOT NULL,
  expected_date TIMESTAMP NOT NULL,
  wave BOOLEAN,
  sunrise TIME WITHOUT TIME zone NOT NULL,
  sunset TIME WITHOUT TIME zone NOT NULL,
  gust REAL NOT NULL,
  wind_speed REAL NOT NULL,
  wind_direction INT NOT NULL,
  temperature REAL NOT NULL,
  FOREIGN KEY (id_model)
    REFERENCES models (id)
);

