UPDATE
  publishers
SET
  publisher = $1
WHERE
  id = $2 RETURNING $table_fields;
