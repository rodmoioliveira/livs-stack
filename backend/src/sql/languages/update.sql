UPDATE
  languages
SET
  language = $1
WHERE
  id = $2 RETURNING $table_fields;
