DELETE FROM
  publishers
WHERE
  id = $1 RETURNING $table_fields;
