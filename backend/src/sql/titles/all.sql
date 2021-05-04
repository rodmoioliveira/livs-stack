SELECT
  *,
  COUNT(*) OVER() as count
FROM
  titles
$filters
$order_by;
