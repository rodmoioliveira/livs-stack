UPDATE
  titles
SET
  isbn = $1, author = $2, title = $3, year = $4, genre_id = $5, publisher_id = $6
WHERE
  isbn = $7 RETURNING $table_fields;
