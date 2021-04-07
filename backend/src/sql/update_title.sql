UPDATE
  titles
SET
  isbn = $1, author = $2, title = $3, publisher = $4, year = $5
WHERE
  isbn = $6 RETURNING $table_fields;
