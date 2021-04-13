UPDATE
  titles
SET
  isbn = $1,
  author = $2,
  edition = $3,
  format = $4,
  language = $5,
  genre = $6,
  pages = $7,
  publisher = $8,
  summary = $9,
  title = $10,
  year = $11
WHERE
  id = $12 RETURNING $table_fields;
