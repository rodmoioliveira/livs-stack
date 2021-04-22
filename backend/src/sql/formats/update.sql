UPDATE
  formats
SET
  format = $1
WHERE
  id = $2 RETURNING $table_fields;
