DELETE FROM
  customers
WHERE
  id = $1 RETURNING $table_fields;
