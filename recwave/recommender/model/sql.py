RECALL_SQL = """
with recent_history as (
 select userid, itemid from user_history_mv
 where timestamp_ > (
 select max(timestamp_) from user_history_mv
 ) - 604800000
 ), counts as (
 select userid, itemid, count(itemid) as count
 from recent_history
 group by userid, itemid
order by userid, count desc
 ) select itemid, count from counts where userid=%s limit 20;
"""

GET_MOST_INTERACTED = """
select * from user_most_interacted_item where userid=%s;
"""
