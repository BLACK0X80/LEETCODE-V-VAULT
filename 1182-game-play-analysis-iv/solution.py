import pandas as pd
def gameplay_analysis(activity: pd.DataFrame) -> pd.DataFrame:
    first = activity.groupby('player_id')['event_date'].min().reset_index()
    first['next'] = first['event_date'] + pd.Timedelta(days=1)
    merged = first.merge(activity, left_on=['player_id','next'], right_on=['player_id','event_date'])
    fraction = round(len(merged) / activity['player_id'].nunique(), 2)
    return pd.DataFrame({'fraction': [fraction]})
