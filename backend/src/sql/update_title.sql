UPDATE
  titles
SET
  isbn = $1, author = $2, title = $3, publisher = $4, year = $5
WHERE
  isbn = $1 RETURNING $table_fields;
