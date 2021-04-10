/* Inspired by https://github.com/mkondratek/Bookstore-Database-Design */

BEGIN TRANSACTION;

/*
 * ===========================
 * enums
 * ===========================
 */

CREATE TYPE format AS ENUM ('hardcover', 'paperback');

/*
 * ===========================
 * languages
 * ===========================
 */

CREATE TABLE IF NOT EXISTS languages (
  id BIGSERIAL PRIMARY KEY,
  language VARCHAR(255) UNIQUE NOT NULL
);

COPY languages(id, language)
FROM
  '/csv/languages.csv' DELIMITER ',' CSV HEADER;

/*
 * ===========================
 * genres
 * ===========================
 */

CREATE TABLE IF NOT EXISTS genres (
  id BIGSERIAL PRIMARY KEY,
  genre VARCHAR(255) UNIQUE NOT NULL
);

COPY genres(id, genre)
FROM
  '/csv/genres.csv' DELIMITER ',' CSV HEADER;

/*
 * ===========================
 * publishers
 * ===========================
 */

CREATE TABLE IF NOT EXISTS publishers (
  id BIGSERIAL PRIMARY KEY,
  publisher VARCHAR(255) UNIQUE NOT NULL
);

COPY publishers(id, publisher)
FROM
  '/csv/publishers.csv' DELIMITER ',' CSV HEADER;

/*
 * ===========================
 * authors
 * ===========================
 */

CREATE TABLE IF NOT EXISTS authors (
  id BIGSERIAL PRIMARY KEY,
  first_name VARCHAR(100) NOT NULL,
  last_name VARCHAR(100) NOT NULL,
  UNIQUE (first_name, last_name)
);

COPY authors(id, first_name, last_name)
FROM
  '/csv/authors.csv' DELIMITER ',' CSV HEADER;

/*
 * ===========================
 * titles
 * ===========================
 */

CREATE TABLE IF NOT EXISTS titles (
  id BIGSERIAL PRIMARY KEY,
  isbn VARCHAR NOT NULL UNIQUE,
  author BIGSERIAL REFERENCES authors(id) ON DELETE CASCADE,
  edition SMALLINT NOT NULL,
  format FORMAT NOT NULL,
  language BIGSERIAL REFERENCES languages(id) ON DELETE CASCADE,
  pages SMALLINT NOT NULL,
  publisher BIGSERIAL REFERENCES publishers(id) ON DELETE CASCADE,
  summary TEXT NOT NULL,
  title VARCHAR(255) NOT NULL,
  year SMALLINT NOT NULL
);

COPY titles(id, isbn, author, edition, format, language, pages, publisher, summary, title, year)
FROM
  '/csv/titles.csv' DELIMITER ',' CSV HEADER;

/*
 * ===========================
 * titles_genres
 * ===========================
 */

CREATE TABLE IF NOT EXISTS titles_genres (
  title_id BIGSERIAL PRIMARY KEY,
  genre_id BIGSERIAL REFERENCES genres(id) ON DELETE CASCADE,
  CONSTRAINT fk_title_id FOREIGN KEY (title_id) REFERENCES titles(id) ON DELETE CASCADE
);

COPY titles_genres(title_id, genre_id)
FROM
  '/csv/titles_genres.csv' DELIMITER ',' CSV HEADER;

/*
 * ===========================
 * measures
 * ===========================
 */

CREATE TABLE IF NOT EXISTS measures (
  title_id BIGINT PRIMARY KEY,
  weight REAL NOT NULL,
  height REAL NOT NULL,
  width REAL NOT NULL,
  depth REAL NOT NULL,
  CONSTRAINT fk_title_id FOREIGN KEY (title_id) REFERENCES titles(id) ON DELETE CASCADE
);

COPY measures(title_id, weight, height, width, depth)
FROM
  '/csv/measures.csv' DELIMITER ',' CSV HEADER;


/*
 * ===========================
 * VIEWS
 * ===========================
 */

CREATE OR REPLACE VIEW genres_count AS (
  SELECT genres.genre,
    COUNT(titles_genres.title_id) AS count
    FROM genres
      JOIN titles_genres ON genres.id = titles_genres.genre_id
    GROUP BY genres.genre
    ORDER BY count DESC
);

/* https://stackoverflow.com/questions/244243/how-to-reset-postgres-primary-key-sequence-when-it-falls-out-of-sync */
SELECT setval(
  pg_get_serial_sequence('titles', 'id'),
  COALESCE(max(id) + 1, 1),
  false
) FROM titles;

SELECT setval(
  pg_get_serial_sequence('genres', 'id'),
  COALESCE(max(id) + 1, 1),
  false
) FROM genres;

SELECT setval(
  pg_get_serial_sequence('publishers', 'id'),
  COALESCE(max(id) + 1, 1),
  false
) FROM publishers;

SELECT setval(
  pg_get_serial_sequence('languages', 'id'),
  COALESCE(max(id) + 1, 1),
  false
) FROM languages;

SELECT setval(
  pg_get_serial_sequence('authors', 'id'),
  COALESCE(max(id) + 1, 1),
  false
) FROM authors;


COMMIT TRANSACTION;
