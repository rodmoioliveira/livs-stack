UPDATE
  reviews
SET
  title_id = $1,
  customer_id = $2,
  review = $3,
  rate = $4
WHERE
  id = $5 RETURNING $table_fields;
