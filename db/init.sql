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

DROP VIEW IF EXISTS reviews_titles;
DROP VIEW IF EXISTS reviews_join_customers;
DROP VIEW IF EXISTS copies_new;
DROP VIEW IF EXISTS copies_used;
DROP VIEW IF EXISTS genres_count;
DROP VIEW IF EXISTS inventory_quantities;
DROP VIEW IF EXISTS titles_info;

DROP FUNCTION IF EXISTS RANDOM_INT;
DROP FUNCTION IF EXISTS RANDOM_TEXT;

BEGIN TRANSACTION;

/*
 * ===========================
 * FUNCTIONS
 * ===========================
 */

/* https://www.postgresqltutorial.com/postgresql-random-range/ */
CREATE OR REPLACE FUNCTION RANDOM_INT(low INT ,high INT)
  RETURNS INT AS
$$
BEGIN
  RETURN floor(random()* (high-low + 1) + low);
END;
$$ language 'plpgsql' STRICT;

/* https://www.simononsoftware.com/random-string-in-postgresql/ */
CREATE OR REPLACE FUNCTION RANDOM_TEXT(INTEGER)
  RETURNS TEXT
  LANGUAGE SQL AS
$$
SELECT LOWER(
  SUBSTRING(
    (SELECT string_agg(md5(random()::TEXT), '')
      FROM generate_series(
        1,
        CEIL($1 / 32.)::integer)
    ), 1, $1) );
$$;

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

INSERT INTO publishers(
  publisher
)
SELECT
  CONCAT (RANDOM_TEXT(5), ' ', RANDOM_TEXT(5))
FROM GENERATE_SERIES(1, 1000) s(i);

/* COPY publishers(id, publisher) */
/* FROM */
/*   '/csv/publishers.csv' DELIMITER ',' CSV HEADER; */

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

INSERT INTO authors(
  first_name,
  last_name
)
SELECT
  RANDOM_TEXT(6),
  RANDOM_TEXT(8)
FROM GENERATE_SERIES(1, 3000) s(i);

/* COPY authors(id, first_name, last_name) */
/* FROM */
/*   '/csv/authors.csv' DELIMITER ',' CSV HEADER; */

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
  cover VARCHAR(100) NOT NULL,
  year SMALLINT NOT NULL
);

INSERT INTO titles(
  isbn,
  author,
  edition,
  format,
  language,
  genre,
  pages,
  publisher,
  summary,
  title,
  cover,
  year
)
SELECT
  LEFT(MD5(RANDOM()::text), 10),
  RANDOM_INT(1,3000),
  RANDOM_INT(1,10),
  RANDOM_INT(1,10),
  RANDOM_INT(1,7),
  RANDOM_INT(1,32),
  RANDOM_INT(50,700),
  RANDOM_INT(1,1000),
  'Lorem ipsum dolor sit amet, consectetur adipiscing elit. Suspendisse consectetur suscipit lacinia. Suspendisse commodo lacus auctor mi aliquet, tristique accumsan est feugiat. Ut felis ex, interdum at consequat et, interdum vel quam. Vivamus nec lectus lacus. Nam interdum lorem eu nulla facilisis, ut vulputate ligula commodo. Donec sagittis scelerisque elementum. Vivamus ultrices volutpat est non commodo.

Morbi sodales nibh vel rutrum tempor. Sed porta rutrum nisi, at vehicula nulla lacinia nec. Vestibulum ultrices velit quis est finibus aliquam. Nulla varius rutrum mauris, non blandit nisi dapibus at. Nunc ultricies placerat ante eu ornare. Integer a pretium sapien. Donec commodo libero euismod elit porta, eu cursus nisl iaculis.',
  CONCAT_WS (' ', RANDOM_TEXT(5), RANDOM_TEXT(5)),
  (
    CASE MOD(i, 156)
      WHEN 0   THEN '420x560,ffebee.png'
      WHEN 1   THEN '420x560,ffcdd2.png'
      WHEN 2   THEN '420x560,ef9a9a.png'
      WHEN 3   THEN '420x560,e57373.png'
      WHEN 4   THEN '420x560,ef5350.png'
      WHEN 5   THEN '420x560,f44336.png'
      WHEN 6   THEN '420x560,ff8a80.png'
      WHEN 7   THEN '420x560,ff5252.png'
      WHEN 8   THEN '420x560,ff1744.png'
      WHEN 9   THEN '420x560,fce4ec.png'
      WHEN 10  THEN '420x560,f8bbd0.png'
      WHEN 11  THEN '420x560,f48fb1.png'
      WHEN 12  THEN '420x560,f06292.png'
      WHEN 13  THEN '420x560,ec407a.png'
      WHEN 14  THEN '420x560,e91e63.png'
      WHEN 15  THEN '420x560,ff80ab.png'
      WHEN 16  THEN '420x560,ff4081.png'
      WHEN 17  THEN '420x560,f50057.png'
      WHEN 18  THEN '420x560,f3e5f5.png'
      WHEN 19  THEN '420x560,e1bee7.png'
      WHEN 20  THEN '420x560,ce93d8.png'
      WHEN 21  THEN '420x560,ba68c8.png'
      WHEN 22  THEN '420x560,ab47bc.png'
      WHEN 23  THEN '420x560,9c27b0.png'
      WHEN 24  THEN '420x560,ea80fc.png'
      WHEN 25  THEN '420x560,e040fb.png'
      WHEN 26  THEN '420x560,d500f9.png'
      WHEN 27  THEN '420x560,ede7f6.png'
      WHEN 28  THEN '420x560,d1c4e9.png'
      WHEN 29  THEN '420x560,b39ddb.png'
      WHEN 30  THEN '420x560,9575cd.png'
      WHEN 31  THEN '420x560,7e57c2.png'
      WHEN 32  THEN '420x560,673ab7.png'
      WHEN 33  THEN '420x560,b388ff.png'
      WHEN 34  THEN '420x560,7c4dff.png'
      WHEN 35  THEN '420x560,651fff.png'
      WHEN 36  THEN '420x560,e8eaf6.png'
      WHEN 37  THEN '420x560,c5cae9.png'
      WHEN 38  THEN '420x560,9fa8da.png'
      WHEN 39  THEN '420x560,7986cb.png'
      WHEN 40  THEN '420x560,5c6bc0.png'
      WHEN 41  THEN '420x560,3f51b5.png'
      WHEN 42  THEN '420x560,8c9eff.png'
      WHEN 43  THEN '420x560,536dfe.png'
      WHEN 44  THEN '420x560,3d5afe.png'
      WHEN 45  THEN '420x560,e3f2fd.png'
      WHEN 46  THEN '420x560,bbdefb.png'
      WHEN 47  THEN '420x560,90caf9.png'
      WHEN 48  THEN '420x560,64b5f6.png'
      WHEN 49  THEN '420x560,42a5f5.png'
      WHEN 50  THEN '420x560,2196f3.png'
      WHEN 51  THEN '420x560,82b1ff.png'
      WHEN 52  THEN '420x560,448aff.png'
      WHEN 53  THEN '420x560,2979ff.png'
      WHEN 54  THEN '420x560,e1f5fe.png'
      WHEN 55  THEN '420x560,b3e5fc.png'
      WHEN 56  THEN '420x560,81d4fa.png'
      WHEN 57  THEN '420x560,4fc3f7.png'
      WHEN 58  THEN '420x560,29b6f6.png'
      WHEN 59  THEN '420x560,03a9f4.png'
      WHEN 60  THEN '420x560,80d8ff.png'
      WHEN 61  THEN '420x560,40c4ff.png'
      WHEN 62  THEN '420x560,00b0ff.png'
      WHEN 63  THEN '420x560,e0f7fa.png'
      WHEN 64  THEN '420x560,b2ebf2.png'
      WHEN 65  THEN '420x560,80deea.png'
      WHEN 66  THEN '420x560,4dd0e1.png'
      WHEN 67  THEN '420x560,26c6da.png'
      WHEN 68  THEN '420x560,00bcd4.png'
      WHEN 69  THEN '420x560,84ffff.png'
      WHEN 70  THEN '420x560,18ffff.png'
      WHEN 71  THEN '420x560,00e5ff.png'
      WHEN 72  THEN '420x560,e0f2f1.png'
      WHEN 73  THEN '420x560,b2dfdb.png'
      WHEN 74  THEN '420x560,80cbc4.png'
      WHEN 75  THEN '420x560,4db6ac.png'
      WHEN 76  THEN '420x560,26a69a.png'
      WHEN 77  THEN '420x560,009688.png'
      WHEN 78  THEN '420x560,a7ffeb.png'
      WHEN 79  THEN '420x560,64ffda.png'
      WHEN 80  THEN '420x560,1de9b6.png'
      WHEN 81  THEN '420x560,e8f5e9.png'
      WHEN 82  THEN '420x560,c8e6c9.png'
      WHEN 83  THEN '420x560,a5d6a7.png'
      WHEN 84  THEN '420x560,81c784.png'
      WHEN 85  THEN '420x560,66bb6a.png'
      WHEN 86  THEN '420x560,4caf50.png'
      WHEN 87  THEN '420x560,b9f6ca.png'
      WHEN 88  THEN '420x560,69f0ae.png'
      WHEN 89  THEN '420x560,00e676.png'
      WHEN 90  THEN '420x560,f1f8e9.png'
      WHEN 91  THEN '420x560,dcedc8.png'
      WHEN 92  THEN '420x560,c5e1a5.png'
      WHEN 93  THEN '420x560,aed581.png'
      WHEN 94  THEN '420x560,9ccc65.png'
      WHEN 95  THEN '420x560,8bc34a.png'
      WHEN 96  THEN '420x560,ccff90.png'
      WHEN 97  THEN '420x560,b2ff59.png'
      WHEN 98  THEN '420x560,76ff03.png'
      WHEN 99  THEN '420x560,f9fbe7.png'
      WHEN 100 THEN '420x560,f0f4c3.png'
      WHEN 101 THEN '420x560,e6ee9c.png'
      WHEN 102 THEN '420x560,dce775.png'
      WHEN 103 THEN '420x560,d4e157.png'
      WHEN 104 THEN '420x560,cddc39.png'
      WHEN 105 THEN '420x560,f4ff81.png'
      WHEN 106 THEN '420x560,eeff41.png'
      WHEN 107 THEN '420x560,c6ff00.png'
      WHEN 108 THEN '420x560,fffde7.png'
      WHEN 109 THEN '420x560,fff9c4.png'
      WHEN 110 THEN '420x560,fff59d.png'
      WHEN 111 THEN '420x560,fff176.png'
      WHEN 112 THEN '420x560,ffee58.png'
      WHEN 113 THEN '420x560,ffeb3b.png'
      WHEN 114 THEN '420x560,ffff8d.png'
      WHEN 115 THEN '420x560,ffff00.png'
      WHEN 116 THEN '420x560,ffea00.png'
      WHEN 117 THEN '420x560,fff8e1.png'
      WHEN 118 THEN '420x560,ffecb3.png'
      WHEN 119 THEN '420x560,ffe082.png'
      WHEN 120 THEN '420x560,ffd54f.png'
      WHEN 121 THEN '420x560,ffca28.png'
      WHEN 122 THEN '420x560,ffc107.png'
      WHEN 123 THEN '420x560,ffe57f.png'
      WHEN 124 THEN '420x560,ffd740.png'
      WHEN 125 THEN '420x560,ffc400.png'
      WHEN 126 THEN '420x560,fff3e0.png'
      WHEN 127 THEN '420x560,ffe0b2.png'
      WHEN 128 THEN '420x560,ffcc80.png'
      WHEN 129 THEN '420x560,ffb74d.png'
      WHEN 130 THEN '420x560,ffa726.png'
      WHEN 131 THEN '420x560,ff9800.png'
      WHEN 132 THEN '420x560,ffd180.png'
      WHEN 133 THEN '420x560,ffab40.png'
      WHEN 134 THEN '420x560,ff9100.png'
      WHEN 135 THEN '420x560,fbe9e7.png'
      WHEN 136 THEN '420x560,ffccbc.png'
      WHEN 137 THEN '420x560,ffab91.png'
      WHEN 138 THEN '420x560,ff8a65.png'
      WHEN 139 THEN '420x560,ff7043.png'
      WHEN 140 THEN '420x560,ff5722.png'
      WHEN 141 THEN '420x560,ff9e80.png'
      WHEN 142 THEN '420x560,ff6e40.png'
      WHEN 143 THEN '420x560,ff3d00.png'
      WHEN 144 THEN '420x560,fafafa.png'
      WHEN 145 THEN '420x560,f5f5f5.png'
      WHEN 146 THEN '420x560,eeeeee.png'
      WHEN 147 THEN '420x560,e0e0e0.png'
      WHEN 148 THEN '420x560,bdbdbd.png'
      WHEN 149 THEN '420x560,9e9e9e.png'
      WHEN 150 THEN '420x560,eceff1.png'
      WHEN 151 THEN '420x560,cfd8dc.png'
      WHEN 152 THEN '420x560,b0bec5.png'
      WHEN 153 THEN '420x560,90a4ae.png'
      WHEN 154 THEN '420x560,78909c.png'
      WHEN 155 THEN '420x560,607d8b.png'
    END
  ),
  RANDOM_INT(1977,2021)
FROM GENERATE_SERIES(1, 10000) s(i);

/* COPY titles(id, isbn, author, edition, format, language, genre, pages, publisher, summary, title, year) */
/* FROM */
/*   '/csv/titles.csv' DELIMITER ',' CSV HEADER; */

/*
 * ===========================
 * inventory
 * ===========================
 */

CREATE TABLE IF NOT EXISTS inventory (
    id BIGSERIAL PRIMARY KEY,
    title_id BIGSERIAL REFERENCES titles(id) ON DELETE CASCADE,
    price REAL NOT NULL,
    quantity BIGINT NOT NULL CHECK (quantity >= 0),
    used BOOLEAN NOT NULL DEFAULT FALSE,
    sku VARCHAR(100) DEFAULT 'SKU_NEW_BOOK' NOT NULL,
    condition TEXT,
    UNIQUE (title_id, used, sku)
);

INSERT INTO inventory(title_id, price, quantity)
SELECT
  i, RANDOM_INT(8,100), RANDOM_INT(1,50)
FROM GENERATE_SERIES(1, 10000) s(i);

INSERT INTO inventory(title_id, price, quantity, used, sku, condition)
SELECT
  i, RANDOM_INT(8,100), 1, TRUE, CONCAT ('SKU_USED_BOOK', '_', i), RANDOM_TEXT(10)
FROM GENERATE_SERIES(1, 10000) s(i);

INSERT INTO inventory(title_id, price, quantity, used, sku, condition)
SELECT
  i, RANDOM_INT(8,100), 1, TRUE, CONCAT ('SKU_USED_BOOK', '_', i + 10000), RANDOM_TEXT(10)
FROM GENERATE_SERIES(1, 10000) s(i);

INSERT INTO inventory(title_id, price, quantity, used, sku, condition)
SELECT
  i, RANDOM_INT(8,100), 1, TRUE, CONCAT ('SKU_USED_BOOK', '_', i + 20000), RANDOM_TEXT(10)
FROM GENERATE_SERIES(1, 10000) s(i);

/* COPY inventory(id, title_id, price, quantity, used, sku, condition) */
/* FROM */
/*   '/csv/inventory.csv' DELIMITER ',' CSV HEADER; */

/*
 * ===========================
 * measures
 * ===========================
 */

CREATE TABLE IF NOT EXISTS measures (
  id BIGSERIAL PRIMARY KEY,
  title_id BIGINT UNIQUE NOT NULL,
  weight REAL NOT NULL,
  height REAL NOT NULL,
  width REAL NOT NULL,
  depth REAL NOT NULL,
  CONSTRAINT fk_title_id FOREIGN KEY (title_id) REFERENCES titles(id) ON DELETE CASCADE
);

INSERT INTO measures(
  title_id,
  weight,
  height,
  width,
  depth
)
SELECT
  i,
  RANDOM_INT(1,50),
  RANDOM_INT(1,50),
  RANDOM_INT(1,50),
  RANDOM_INT(1,50)
FROM GENERATE_SERIES(1, 10000) s(i);

/* COPY measures(title_id, weight, height, width, depth) */
/* FROM */
/*   '/csv/measures.csv' DELIMITER ',' CSV HEADER; */

/*
 * ===========================
 * customers
 * ===========================
 */

CREATE TABLE IF NOT EXISTS customers (
  id BIGSERIAL PRIMARY KEY,
  first_name VARCHAR(100) NOT NULL,
  last_name VARCHAR(100) NOT NULL,
  email VARCHAR(100) UNIQUE NOT NULL
);

INSERT INTO customers(
  first_name,
  last_name,
  email
)
SELECT
  RANDOM_TEXT(6),
  RANDOM_TEXT(6),
  RANDOM_TEXT(15)
FROM GENERATE_SERIES(1, 20000) s(i);

/* COPY customers(id, first_name, last_name, email) */
/* FROM */
/*   '/csv/customers.csv' DELIMITER ',' CSV HEADER; */

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

INSERT INTO reviews(title_id, customer_id, review, rate)
SELECT i, i, RANDOM_TEXT(10), RANDOM_INT(1,5) FROM GENERATE_SERIES(1, 10000) s(i);
INSERT INTO reviews(title_id, customer_id, review, rate)
SELECT i, i + 1, RANDOM_TEXT(10), RANDOM_INT(1,5) FROM GENERATE_SERIES(1, 9999) s(i);
INSERT INTO reviews(title_id, customer_id, review, rate)
SELECT i, i + 2, RANDOM_TEXT(10), RANDOM_INT(1,5) FROM GENERATE_SERIES(1, 9998) s(i);
INSERT INTO reviews(title_id, customer_id, review, rate)
SELECT i, i + 3, RANDOM_TEXT(10), RANDOM_INT(1,5) FROM GENERATE_SERIES(1, 9997) s(i);
INSERT INTO reviews(title_id, customer_id, review, rate)
SELECT i, i + 4, RANDOM_TEXT(10), RANDOM_INT(1,5) FROM GENERATE_SERIES(1, 9996) s(i);

/* COPY reviews(id, title_id, customer_id, review, rate) */
/* FROM */
/*   '/csv/reviews.csv' DELIMITER ',' CSV HEADER; */

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

CREATE OR REPLACE VIEW reviews_titles as (
  SELECT
    title_id,
    round(avg(rate)) AS rate,
    count(*) AS count
  FROM reviews
  GROUP BY title_id
  ORDER BY rate DESC
);

CREATE OR REPLACE VIEW reviews_join_customers as (
  SELECT
    reviews.id,
    reviews.title_id,
    reviews.customer_id,
    CONCAT (customers.first_name, ' ', customers.last_name) AS customer_name,
    reviews.review,
    reviews.rate
  FROM reviews
    JOIN customers ON reviews.customer_id = customers.id
  ORDER BY reviews.id
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
    inventory_quantities.total as copies_total,
    reviews_titles.rate as reviews_rate,
    reviews_titles.count as reviews_count
  FROM titles
    JOIN authors ON titles.author = authors.id
    JOIN formats ON titles.format = formats.id
    JOIN languages ON titles.language = languages.id
    JOIN measures ON titles.id = measures.title_id
    JOIN publishers ON publishers.id = titles.publisher
    JOIN genres ON titles.genre = genres.id
    JOIN inventory_quantities ON titles.id = inventory_quantities.title_id
    JOIN reviews_titles ON titles.id = reviews_titles.title_id
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
