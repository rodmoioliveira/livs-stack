INSERT INTO
  measures(weight, height, width, depth, title_id)
VALUES
  ($1, $2, $3, $4, $5) RETURNING $table_fields;
