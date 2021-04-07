DELETE FROM
  titles
WHERE
  isbn = $1 RETURNING $table_fields;
