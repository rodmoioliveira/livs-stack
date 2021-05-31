INSERT INTO
  publishers(publisher)
VALUES
  ($1) RETURNING $table_fields;
