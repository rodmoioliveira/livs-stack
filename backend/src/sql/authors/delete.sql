DELETE FROM
  authors
WHERE
  id = $1 RETURNING $table_fields;
