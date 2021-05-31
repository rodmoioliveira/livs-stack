DELETE FROM
  reviews
WHERE
  id = $1 RETURNING $table_fields;
