DELETE FROM
  inventory
WHERE
  id = $1 RETURNING $table_fields;
