INSERT INTO
  titles(isbn, author, title, year, genre_id, publisher_id)
VALUES
  ($1, $2, $3, $4, $5, $6) RETURNING $table_fields;
