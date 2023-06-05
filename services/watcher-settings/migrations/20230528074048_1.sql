CREATE TABLE IF NOT EXISTS watcher-settings {
  id_spot INT
  id_station INT
  PRIMARY KEY (id_spot, id_station)
}

CREATE TABLE IF NOT EXISTS spots (
  id INT PRIMARY KEY,
  name VARCHAR(255) UNIQUE NOT NULL,
  country VARCHAR(255) NOT NULL,
  gmt_hour_offset INT NOT NULL,
  models INTEGER[]
);
