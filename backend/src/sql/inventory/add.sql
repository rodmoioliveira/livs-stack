INSERT INTO
  inventory(title_id, price, quantity, used, sku, condition)
VALUES
  ($1, $2, $3, $4, $5, $6) RETURNING $table_fields;
