create source if not exists actionhistory (
    userid int,
    itemid int,
    action int,
    timestamp_ int,
) with (
    connector='kafka',
    kafka.topic='recwave',
    kafka.brokers='127.0.0.1:9092',
    kafka.consumer.group='recwave-recommender'
)
row format json;

create table if not exists user (
  id integer,
  address_lat numeric,
  address_long numeric,  -- datatype `point` not implemented
  age_approx integer,
  gender integer,
  occupation numeric,
  -- and more ...
);

create table if not exists item (
  id integer,
  category integer,
  brand integer,
  freshness numeric,
  popularity numeric,
  price numeric,
  rating numeric
  -- and more ...
);

create materialized view user_history_mv as
    select * from actionhistory;

create materialized view user_most_interacted_item as
with counts as (select userid, itemid, count(itemid) as count
    from actionhistory
    group by userid, itemid
    order by userid, count desc)
select userid, max((count, itemid)) as maxcount_item from counts group by userid;
