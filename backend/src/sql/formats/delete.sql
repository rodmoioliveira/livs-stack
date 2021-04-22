DELETE FROM
  formats
WHERE
  id = $1 RETURNING $table_fields;
