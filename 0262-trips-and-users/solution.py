import pandas as pd

def trips_and_users(trips: pd.DataFrame, users: pd.DataFrame) -> pd.DataFrame:
    banned = set(users[users['banned'] == 'Yes']['users_id'])

    mask = (
        ~trips['client_id'].isin(banned) &
        ~trips['driver_id'].isin(banned) &
        (trips['request_at'] >= '2013-10-01') &
        (trips['request_at'] <= '2013-10-03')
    )
    filtered = trips[mask].copy()

    filtered['cancelled'] = filtered['status'] != 'completed'

    result = filtered.groupby('request_at').agg(
        cancel_count=('cancelled', 'sum'),
        total=('cancelled', 'count')
    ).reset_index()

    result['Cancellation Rate'] = (result['cancel_count'] / result['total']).round(2)

    return result[['request_at', 'Cancellation Rate']].rename(
        columns={'request_at': 'Day'}
    )
