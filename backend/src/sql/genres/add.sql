INSERT INTO
  genres(genre)
VALUES
  ($1) RETURNING $table_fields;
