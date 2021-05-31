UPDATE
  authors
SET
  first_name = $1,
  last_name = $2
WHERE
  id = $3 RETURNING $table_fields;
