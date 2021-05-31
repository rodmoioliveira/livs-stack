UPDATE
  measures
SET
  weight = $1,
  height = $2,
  width = $3,
  depth = $4
WHERE
  title_id = $5 RETURNING $table_fields;
