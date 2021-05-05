WITH
cte AS (
  SELECT
    *
  FROM
    titles
  $filters
  )
SELECT *
FROM (
  TABLE cte
  $order_by
  ) sub
RIGHT JOIN (
  SELECT count(*)
  FROM cte
) c(count)
ON TRUE;
