create table person (
    "id" int,
    "name" varchar,
    "email_address" varchar,
    "credit_card" varchar,
    "city" varchar,
    PRIMARY KEY ("id")
) with (
    connector = 'postgres-cdc',
    hostname = 'postgres',
    port = '5432',
    username = 'myuser',
    password = '123456',
    database.name = 'mydb',
    schema.name = 'public',
    table.name = 'orders',
    slot.name = 'orders'
);