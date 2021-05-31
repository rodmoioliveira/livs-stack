UPDATE
  customers
SET
  first_name = $1,
  last_name = $2,
  email = $3
WHERE
  id = $4 RETURNING $table_fields;
