import pandas as pd

def user_activity(activity: pd.DataFrame) -> pd.DataFrame:
    black = activity[(activity['activity_date'] <= '2019-07-27') & (activity['activity_date'] > '2019-06-27')]
    black = black.groupby('activity_date')['user_id'].nunique().reset_index()
    return black.rename(columns={'activity_date': 'day', 'user_id': 'active_users'})
    
