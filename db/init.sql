/* Inspired by https://github.com/mkondratek/Bookstore-Database-Design */

BEGIN TRANSACTION;

/*
 * ===========================
 * enums
 * ===========================
 */

/* CREATE TYPE format AS ENUM ('hardcover', 'paperback'); */

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
  last_name VARCHAR(100) NOT NULL
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
  format VARCHAR(10) NOT NULL CHECK (format = 'paperback' OR format = 'hardcover'),
  language BIGSERIAL REFERENCES languages(id) ON DELETE CASCADE,
  genre BIGSERIAL REFERENCES genres(id) ON DELETE CASCADE,
  pages SMALLINT NOT NULL,
  publisher BIGSERIAL REFERENCES publishers(id) ON DELETE CASCADE,
  summary TEXT NOT NULL,
  title VARCHAR(255) NOT NULL,
  year SMALLINT NOT NULL
);

COPY titles(id, isbn, author, edition, format, language, genre, pages, publisher, summary, title, year)
FROM
  '/csv/titles.csv' DELIMITER ',' CSV HEADER;

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

CREATE OR REPLACE VIEW genres_count as (
  SELECT
    genres.genre,
    count(*) as count
  FROM titles
  JOIN genres on titles.genre = genres.id
  GROUP BY genres.genre
  ORDER BY count DESC
);

CREATE OR REPLACE VIEW titles_info as (
  SELECT
    titles.id,
    titles.isbn,
    CONCAT (authors.first_name, ' ', authors.last_name) AS author,
    titles.edition,
    titles.format,
    languages.language,
    titles.pages,
    publishers.publisher,
    titles.summary,
    titles.title,
    titles.year,
    measures.weight,
    measures.height,
    measures.width,
    measures.depth,
    genres.genre
  FROM titles
    JOIN authors ON titles.author = authors.id
    JOIN languages ON titles.language = languages.id
    JOIN measures ON titles.id = measures.title_id
    JOIN publishers ON publishers.id = titles.publisher
    JOIN genres ON titles.genre = genres.id
  ORDER BY titles.id
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
