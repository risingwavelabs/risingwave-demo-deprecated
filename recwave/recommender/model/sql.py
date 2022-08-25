"""
todo:
the batch query on source isn't permitted
however, the recall can be based on a roughly calculated score
the score for each action is counted
if so, the get_most_interacted is going to need a better model

"""

RECALL_SQL = """
select itemid from recenthistory where userid = %s limit 20;
"""

GET_MOST_INTERACTED = """
select * from user_most_interacted_item where userid=%s;
"""
