create materialized source orders (
    order_id int,
    order_date bigint,
    customer_name varchar,
    price decimal,
    product_id int,
    order_status smallint,
    PRIMARY KEY (order_id)
) with (
    connector = 'postgres-cdc',
    hostname = 'postgres',
    port = '5432',
    username = 'postgres',
    password = 'postgres',
    database.name = 'cdc_test',
    schema.name = 'public',
    table.name = 'orders',
    slot.name = 'orders'
);