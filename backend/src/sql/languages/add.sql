INSERT INTO
  languages(language)
VALUES
  ($1) RETURNING $table_fields;
