create materialized source orders (
    order_id int,
    order_date bigint,
    customer_name varchar,
    price decimal,
    product_id int,
    order_status smallint,
    PRIMARY KEY (order_id)
) with (
    connector = 'kafka',
    kafka.topic = 'live_stream_metrics',
    kafka.brokers = 'message_queue:29092',
    kafka.scan.startup.mode = 'earliest'
);