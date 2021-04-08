BEGIN TRANSACTION;

CREATE TABLE IF NOT EXISTS genres (
  id BIGSERIAL,
  genre VARCHAR(255),
  PRIMARY KEY (id)
);

COPY genres(id, genre)
FROM
  '/csv/genres.csv' DELIMITER ',' CSV HEADER;

CREATE TABLE IF NOT EXISTS publishers (
  id BIGSERIAL,
  publisher VARCHAR(255),
  PRIMARY KEY (id)
);

COPY publishers(id, publisher)
FROM
  '/csv/publishers.csv' DELIMITER ',' CSV HEADER;

CREATE TABLE IF NOT EXISTS titles (
  id BIGSERIAL,
  isbn BIGINT NOT NULL,
  author VARCHAR(255) NOT NULL,
  title VARCHAR(255) NOT NULL,
  year SMALLINT NOT NULL,
  genre_id BIGINT NOT NULL,
  publisher_id BIGINT NOT NULL,
  PRIMARY KEY (id),
  UNIQUE (isbn),
  FOREIGN KEY (genre_id) REFERENCES genres(id) ON DELETE CASCADE,
  FOREIGN KEY (publisher_id) REFERENCES publishers(id) ON DELETE CASCADE
);

COPY titles(id, isbn, author, title, year, genre_id, publisher_id)
FROM
  '/csv/titles.csv' DELIMITER ',' CSV HEADER;

COMMIT TRANSACTION;
