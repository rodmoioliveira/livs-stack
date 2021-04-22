UPDATE
  genres
SET
  genre = $1
WHERE
  id = $2 RETURNING $table_fields;
