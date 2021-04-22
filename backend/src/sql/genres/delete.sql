DELETE FROM
  genres
WHERE
  id = $1 RETURNING $table_fields;
