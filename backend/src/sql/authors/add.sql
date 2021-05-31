INSERT INTO
  authors(first_name, last_name)
VALUES
  ($1, $2) RETURNING $table_fields;
