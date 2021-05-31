UPDATE
  inventory
SET
  title_id = $1,
  price = $2,
  quantity = $3,
  used = $4,
  sku = $5,
  condition = $6
WHERE
  id = $7 RETURNING $table_fields;
