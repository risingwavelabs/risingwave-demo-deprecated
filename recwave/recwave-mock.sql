-- the user history table
create table actionhistory (
  id integer primary key,
  userid integer,
  itemid integer,
  action integer,
  timestamp_ integer
);


insert into actionhistory values
    (1, 1, 1, 0, 123456789),
    (2, 1, 1, 1, 123456790),
    (3, 1, 2, 0, 123456791),
    (4, 1, 2, 0, 123456792),
    (5, 1, 2, 1, 123456793),
    (6, 2, 1, 0, 123456794),
    (7, 2, 1, 0, 123456795),
    (8, 2, 2, 0, 123456796),
    (9, 2, 2, 0, 123456797),
    (10,2, 2, 0, 123456798),
    (11,2, 2, 1, 123456799),
    (12,2, 2, 1, 123456800);


-- recent most viewed items

create materialized view user_most_interacted_item as
select max(userid), max(itemid), max(count) as max_count from
 (select userid, itemid, count(itemid) as count
 from actionhistory where timestamp_ > max(timestamp_) - 86400000
    group by itemid, userid) max_counts
group by userid, itemid;


-- recent most interacted user for each item
create materialized view item_most_accessed_user as
select max(itemid), max(userid), max(count) as max_count from
 (select userid, itemid, count(itemid) over (partition by itemid) as count
 from actionhistory where timestamp_ > max(timestamp_) - 86400000
    group by itemid, userid) max_counts
group by itemid;

