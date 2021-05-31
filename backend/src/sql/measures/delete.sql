DELETE FROM
  measures
WHERE
  title_id = $1 RETURNING $table_fields;
