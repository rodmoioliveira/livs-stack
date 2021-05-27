INSERT INTO
  titles(
    isbn,
    author,
    edition,
    format,
    language,
    genre,
    pages,
    publisher,
    summary,
    title,
    cover,
    year
  )
VALUES
  ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12) RETURNING $table_fields;
