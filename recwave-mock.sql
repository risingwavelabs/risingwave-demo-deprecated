-- the user history table

create table actionhistory (
  userid integer,
  itemid integer,
  action integer,
  timestamp_ timestamp
);


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


create materialized view recent_history as select *
    from tumble(actionhistory, timestamp_, INTERVAL '30 seconds')
    order by window_end limit 20;


create materialized view user_most_interacted_item as
    with counts as (select userid, itemid, count(itemid) as count, window_start
    from (
        select * from tumble(actionhistory, timestamp_, interval '5 minutes')
    ) recent
    group by userid, itemid
    )
select userid, max((window_start, count, itemid)) as maxcount_item from counts group by userid;


insert into actionhistory values
    (1, 1, 0, '2016-02-01 00:00:01'),
    (1, 1, 1, '2016-02-01 00:00:02'),
    (1, 2, 0, '2016-02-01 00:00:03'),
    (1, 2, 0, '2016-02-01 00:00:04'),
    (1, 2, 1, '2016-02-01 00:00:05'),
    (2, 1, 0, '2016-02-01 00:00:06'),
    (2, 1, 0, '2016-02-01 00:00:07'),
    (2, 2, 0, '2016-02-01 00:00:08'),
    (2, 2, 0, '2016-02-01 00:00:09'),
    (2, 2, 0, '2016-02-01 00:00:10'),
    (2, 2, 1, '2016-02-01 00:00:11'),
    (2, 2, 1, '2016-02-01 00:00:12');