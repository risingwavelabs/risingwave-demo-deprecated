create materialized source orders (
    order_id int,
    order_date bigint,
    customer_name varchar,
    price decimal,
    product_id int,
    order_status smallint,
    PRIMARY KEY (order_id)
) with (
    connector = 'cdc',
    database.hostname = 'mysql',
    database.port = '3306',
    database.user = 'root',
    database.password = '123456',
    database.name = 'mydb',
    table.name = 'orders'
) row format debezium_json;