INSERT INTO
  customers(first_name, last_name, email)
VALUES
  ($1, $2, $3) RETURNING $table_fields;
