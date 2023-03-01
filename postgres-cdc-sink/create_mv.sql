CREATE MATERIALIZED VIEW city_population AS
SELECT
    city,
    COUNT(*) as population
FROM
    person
GROUP BY
    city;

CREATE MATERIALIZED VIEW seller_auctions AS
SELECT
    P.id,
    P.name,
    A.starttime,
    A.auctions_count
FROM
    person as P
    JOIN (
        SELECT
            seller,
            COUNT(*) as auctions_count,
            window_start AS starttime
        FROM
            TUMBLE(auction, date_time, INTERVAL '10' SECOND)
        GROUP BY
            seller,
            window_start
    ) A ON P.id = A.seller;

CREATE SINK seller_auctions_sink
FROM
    target_count WITH (
        connector = 'jdbc',
        jdbc.url = 'jdbc:postgresql://postgres:5432/mydb?user=myuser&password=123456',
        table.name = 'seller_auctions'
    );