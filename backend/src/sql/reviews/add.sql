INSERT INTO
  reviews(title_id, customer_id, review, rate)
VALUES
  ($1, $2, $3, $4) RETURNING $table_fields;
