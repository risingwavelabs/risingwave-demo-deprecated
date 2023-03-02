CREATE MATERIALIZED VIEW seller_auctions AS
SELECT
    P.id as seller_id,
    P.name as seller_name,
    A.date_time as auction_time,
    A.item_name as auction_item_name
    A.category as auction_category
    A.id as auction_id
FROM
    person as P
    JOIN auction as A ON P.id = A.seller;