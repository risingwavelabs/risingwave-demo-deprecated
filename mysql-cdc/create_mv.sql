CREATE MATERIALIZED VIEW orders_mv AS
SELECT
    product_id,
    COUNT(*) as product_count
FROM
    orders
GROUP BY
    product_id;