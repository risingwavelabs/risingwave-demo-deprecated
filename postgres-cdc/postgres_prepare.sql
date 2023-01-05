-- # import data to postgres
-- createdb -U postgres cdc_test
-- psql -U postgres -d cdc_test < postgres_prepare.sql

create table orders (
  order_id int,
  order_date bigint,
  customer_name varchar(200),
  price decimal,
  product_id int,
  order_status smallint,
  PRIMARY KEY (order_id)
);

ALTER SEQUENCE public.orders_order_id_seq RESTART WITH 1001;
ALTER TABLE public.orders REPLICA IDENTITY FULL;

insert into
  orders
values
  (1, 1558430840000, 'Bob', 10.50, 1, 1),
  (2, 1558430840001, 'Alice', 20.50, 2, 1),
  (
    3,
    1558430840002,
    'Alice',
    18.50,
    2,
    1
  );