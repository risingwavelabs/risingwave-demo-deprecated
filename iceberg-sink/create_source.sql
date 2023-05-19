create table t_person (
    "id" int,
    "name" varchar,
    "email_address" varchar,
    "credit_card" varchar,
    "city" varchar,
    "date_time" bigint,
    PRIMARY KEY ("id")
) with (
    connector = 'mysql-cdc',
    hostname = 'mysql',
    port = '3306',
    username = 'root',
    password = '123456',
    database.name = 'mydb',
    table.name = 'person',
    server.id = '1'
);

CREATE TABLE t_auction (
    id BIGINT,
    item_name VARCHAR,
    date_time BIGINT,
    seller INT,
    category INT,
    PRIMARY KEY (id)
) WITH (
    connector = 'kafka',
    topic = 'auction',
    properties.bootstrap.server = 'message_queue:29092',
    scan.startup.mode = 'earliest'
) ROW FORMAT JSON;

CREATE VIEW person as
SELECT
    id,
    name,
    email_address,
    credit_card,
    city,
    to_timestamp(date_time) as date_time
FROM
    t_person;

CREATE VIEW auction as
SELECT
    id,
    item_name,
    to_timestamp(date_time) as date_time,
    seller,
    category
FROM
    t_auction;