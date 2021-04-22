DELETE FROM
  languages
WHERE
  id = $1 RETURNING $table_fields;
