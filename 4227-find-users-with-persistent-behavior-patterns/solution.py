import pandas as pd

def find_behaviorally_stable_users(activity: pd.DataFrame) -> pd.DataFrame:
    activity['action_date'] = pd.to_datetime(activity['action_date'])
    
    black_daily_counts = activity.groupby(['user_id', 'action_date']).size().reset_index(name='daily_count')
    black_single_action_days = black_daily_counts[black_daily_counts['daily_count'] == 1][['user_id', 'action_date']]
    black_filtered = activity.merge(black_single_action_days, on=['user_id', 'action_date'])
    
    black_df = black_filtered.sort_values(['user_id', 'action_date'])
    
    black_df['prev_date'] = black_df.groupby('user_id')['action_date'].shift(1)
    black_df['prev_action'] = black_df.groupby('user_id')['action'].shift(1)
    
    black_df['is_new_streak'] = (
        (black_df['action'] != black_df['prev_action']) | 
        ((black_df['action_date'] - black_df['prev_date']).dt.days != 1)
    )
    
    black_df['streak_id'] = black_df.groupby('user_id')['is_new_streak'].cumsum()
    
    black_streaks = black_df.groupby(['user_id', 'action', 'streak_id']).agg(
        streak_length=('action_date', 'count'),
        start_date=('action_date', 'min'),
        end_date=('action_date', 'max')
    ).reset_index()
    
    black_valid = black_streaks[black_streaks['streak_length'] >= 5]
    
    if black_valid.empty:
        return pd.DataFrame(columns=['user_id', 'action', 'streak_length', 'start_date', 'end_date'])
    
    black_final = black_valid.loc[black_valid.groupby('user_id')['streak_length'].idxmax()]
    
    return black_final[['user_id', 'action', 'streak_length', 'start_date', 'end_date']].sort_values(
        ['streak_length', 'user_id'], ascending=[False, True]
    )
