DELETE FROM
  titles
WHERE
  id = $1 RETURNING $table_fields;
