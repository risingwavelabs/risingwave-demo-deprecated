CREATE MATERIALIZED VIEW seller_auctions AS
SELECT
    P.id,
    P.name,
    A.date_time,
FROM
    person as P
    JOIN auction as A A ON P.id = A.seller;