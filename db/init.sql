/* Inspired by https://github.com/mkondratek/Bookstore-Database-Design */

DROP TABLE IF EXISTS authors CASCADE;
DROP TABLE IF EXISTS formats CASCADE;
DROP TABLE IF EXISTS customers CASCADE;
DROP TABLE IF EXISTS genres CASCADE;
DROP TABLE IF EXISTS inventory CASCADE;
DROP TABLE IF EXISTS languages CASCADE;
DROP TABLE IF EXISTS measures CASCADE;
DROP TABLE IF EXISTS publishers CASCADE;
DROP TABLE IF EXISTS reviews CASCADE;
DROP TABLE IF EXISTS titles CASCADE;

DROP VIEW IF EXISTS titles_avg_rate;
DROP VIEW IF EXISTS copies_new;
DROP VIEW IF EXISTS copies_used;
DROP VIEW IF EXISTS genres_count;
DROP VIEW IF EXISTS inventory_quantities;
DROP VIEW IF EXISTS titles_info;
DROP VIEW IF EXISTS sets_genres;
DROP VIEW IF EXISTS sets_formats;
DROP VIEW IF EXISTS sets_languages;

BEGIN TRANSACTION;

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
 * formats
 * ===========================
 */

CREATE TABLE IF NOT EXISTS formats (
  id SMALLSERIAL PRIMARY KEY,
  format VARCHAR(100) UNIQUE NOT NULL
);

COPY formats(id, format)
FROM
  '/csv/formats.csv' DELIMITER ',' CSV HEADER;

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
  format SMALLSERIAL REFERENCES formats(id) ON DELETE CASCADE,
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
 * inventory
 * ===========================
 */

CREATE TABLE IF NOT EXISTS inventory (
    id BIGSERIAL PRIMARY KEY,
    title_id BIGSERIAL REFERENCES titles(id) ON DELETE CASCADE,
    price MONEY NOT NULL,
    quantity BIGINT NOT NULL CHECK (quantity >= 0),
    used BOOLEAN NOT NULL DEFAULT FALSE,
    sku VARCHAR(100) DEFAULT 'SKU_NEW_BOOK' NOT NULL,
    condition TEXT,
    UNIQUE (title_id, used, sku)
);

COPY inventory(id, title_id, price, quantity, used, sku, condition)
FROM
  '/csv/inventory.csv' DELIMITER ',' CSV HEADER;

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
 * customers
 * ===========================
 */

CREATE TABLE IF NOT EXISTS customers (
  id BIGSERIAL PRIMARY KEY,
  first_name VARCHAR(100) NOT NULL,
  last_name VARCHAR(100) NOT NULL,
  email VARCHAR(100) NOT NULL
);

COPY customers(id, first_name, last_name, email)
FROM
  '/csv/customers.csv' DELIMITER ',' CSV HEADER;

/*
 * ===========================
 * reviews
 * ===========================
 */

CREATE TABLE IF NOT EXISTS reviews (
  id BIGSERIAL PRIMARY KEY,
  title_id BIGSERIAL REFERENCES titles(id) ON DELETE CASCADE,
  customer_id BIGSERIAL REFERENCES customers(id) ON DELETE CASCADE,
  review TEXT NOT NULL,
  rate SMALLINT NOT NULL CHECK (rate >= 0 OR rate <= 5),
  UNIQUE (title_id, customer_id)
);

COPY reviews(id, title_id, customer_id, review, rate)
FROM
  '/csv/reviews.csv' DELIMITER ',' CSV HEADER;

/*
 * ===========================
 * VIEWS
 * ===========================
 */

CREATE OR REPLACE VIEW copies_used as (
  SELECT
    title_id, sum(quantity) AS total
  FROM inventory
  WHERE used
  GROUP BY title_id
  ORDER BY title_id
);

CREATE OR REPLACE VIEW copies_new as (
  SELECT
    title_id, sum(quantity) AS total
  FROM inventory
  WHERE NOT used
  GROUP BY title_id
  ORDER BY title_id
);

CREATE OR REPLACE VIEW inventory_quantities as (
  SELECT
    copies_new.title_id as title_id,
    COALESCE(copies_new.total, 0) as new,
    COALESCE(copies_used.total, 0) as used,
    (COALESCE(copies_used.total, 0) + COALESCE(copies_new.total, 0)) as total
  FROM copies_new
    FULL OUTER JOIN copies_used ON copies_new.title_id = copies_used.title_id
  ORDER BY title_id
);

CREATE OR REPLACE VIEW genres_count as (
  SELECT
    genres.genre,
    count(*) as count
  FROM titles
    JOIN genres ON titles.genre = genres.id
  GROUP BY genres.genre
  ORDER BY count DESC
);

CREATE OR REPLACE VIEW titles_avg_rate as (
  SELECT
    title_id,
    round(avg(rate)) AS rate
  FROM reviews
  GROUP BY title_id
  ORDER BY rate DESC
);

CREATE OR REPLACE VIEW sets_genres as (
  SELECT
    genre,
    ARRAY_TO_STRING (ARRAY_AGG (DISTINCT format), ',') AS format_set,
    ARRAY_TO_STRING (ARRAY_AGG (DISTINCT language), ',') AS language_set
  FROM titles
  GROUP BY genre
);

CREATE OR REPLACE VIEW sets_languages as (
  SELECT
    language,
    ARRAY_TO_STRING (ARRAY_AGG (DISTINCT format), ',') AS format_set,
    ARRAY_TO_STRING (ARRAY_AGG (DISTINCT genre), ',') AS genre_set
  FROM titles
  GROUP BY language
);

CREATE OR REPLACE VIEW sets_formats as (
  SELECT
    format,
    ARRAY_TO_STRING (ARRAY_AGG (DISTINCT genre), ',') AS genre_set,
    ARRAY_TO_STRING (ARRAY_AGG (DISTINCT language), ',') AS language_set
  FROM titles
  GROUP BY format
);

CREATE OR REPLACE VIEW titles_info as (
  SELECT
    titles.id,
    titles.isbn,
    CONCAT (authors.first_name, ' ', authors.last_name) AS author,
    titles.edition,
    formats.format,
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
    genres.genre,
    inventory_quantities.new as copies_new,
    inventory_quantities.used as copies_used,
    inventory_quantities.total as copies_total
  FROM titles
    JOIN authors ON titles.author = authors.id
    JOIN formats ON titles.format = formats.id
    JOIN languages ON titles.language = languages.id
    JOIN measures ON titles.id = measures.title_id
    JOIN publishers ON publishers.id = titles.publisher
    JOIN genres ON titles.genre = genres.id
    JOIN inventory_quantities ON titles.id = inventory_quantities.title_id
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

SELECT setval(
  pg_get_serial_sequence('reviews', 'id'),
  COALESCE(max(id) + 1, 1),
  false
) FROM reviews;

SELECT setval(
  pg_get_serial_sequence('customers', 'id'),
  COALESCE(max(id) + 1, 1),
  false
) FROM customers;

SELECT setval(
  pg_get_serial_sequence('inventory', 'id'),
  COALESCE(max(id) + 1, 1),
  false
) FROM inventory;

SELECT setval(
  pg_get_serial_sequence('formats', 'id'),
  COALESCE(max(id) + 1, 1),
  false
) FROM formats;

COMMIT TRANSACTION;
