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
) sub right join (select count(*) from cte) c(count) on true;
