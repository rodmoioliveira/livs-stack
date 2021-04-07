INSERT INTO titles(isbn, author, title, publisher, year)
VALUES ($1, $2, $3, $4, $5)
RETURNING $table_fields;
