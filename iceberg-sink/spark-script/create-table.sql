drop table if exists demo.demo_db.seller_auctions;

CREATE TABLE demo.demo_db.seller_auctions (
  seller_id string,
  seller_name string,
  auction_time timestamp,
  auction_item_name string,
  auction_category int,
  auction_id int
) TBLPROPERTIES ('format-version' = '2');