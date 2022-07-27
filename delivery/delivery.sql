CREATE SOURCE delivery_orders_source (
    order_id BIGINT,
    restaurant_id BIGINT,
    order_state VARCHAR,
    order_timestamp TIMESTAMP
) WITH (
    'connector' = 'kafka',
    'kafka.topic' = 'delivery_orders',
    'kafka.brokers' = 'message_queue:29092',
    'kafka.scan.startup.mode' = 'earliest'
) ROW FORMAT JSON;


CREATE MATERIALIZED VIEW restaurant_orders AS
SELECT
    window_start,
    restaurant_id,
    COUNT(*) AS total_order
FROM 
    HOP(delivery_orders_source, order_timestamp, INTERVAL '1' MINUTE, INTERVAL '15' MINUTE)
WHERE 
    order_state = 'CREATED'
GROUP BY
    restaurant_id,
    window_start;
