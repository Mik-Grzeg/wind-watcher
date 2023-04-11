CREATE TABLE IF NOT EXISTS models (
  id INT PRIMARY KEY,
  identifier VARCHAR(255),
  name VARCHAR(255) UNIQUE
);

CREATE TABLE IF NOT EXISTS spots (
  id INT PRIMARY KEY,
  name VARCHAR(255) UNIQUE NOT NULL,
  country VARCHAR(255) NOT NULL,
  models INTEGER[]
);

CREATE TABLE IF NOT EXISTS station_readings (
  id_spot INT NOT NULL,
  time TIMESTAMP NOT NULL,
  sunrise TIME WITHOUT TIME zone NOT NULL,
  sunset TIME WITHOUT TIME zone NOT NULL,
  PRIMARY KEY (id_spot, time),
  FOREIGN KEY (id_spot)
    REFERENCES spots (id)
);


CREATE TABLE IF NOT EXISTS forecasts (
  id_spot INT NOT NULL,
  id_model INT NOT NULL,
  forecast_from TIMESTAMP NOT NULL,
  forecast_for TIMESTAMP,
  wave BOOLEAN,
  gust REAL,
  wind_speed REAL,
  wind_direction INT,
  relative_humidity INT,
  cloud_cover_high INT,
  cloud_cover_mid INT,
  cloud_cover_low INT,
  precipitation INT,
  temperature REAL,
  PRIMARY KEY (id_spot, id_model, forecast_from, forecast_for),
  CONSTRAINT fk_model
    FOREIGN KEY (id_model)
      REFERENCES models (id),
  CONSTRAINT fk_spot
    FOREIGN KEY (id_spot)
      REFERENCES spots (id)
);

