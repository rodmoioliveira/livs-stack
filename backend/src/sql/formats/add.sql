INSERT INTO
  formats(format)
VALUES
  ($1) RETURNING $table_fields;
