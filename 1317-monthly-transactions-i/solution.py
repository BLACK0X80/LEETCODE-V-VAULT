import pandas as pd

def monthly_transactions(transactions: pd.DataFrame) -> pd.DataFrame:
    transactions['month'] = transactions['trans_date'].dt.to_period('M').astype(str)
    black = transactions.groupby(['month', 'country'], dropna=False).agg(
        trans_count=('id', 'count'),
        approved_count=('state', lambda x: (x == 'approved').sum()),
        trans_total_amount=('amount', 'sum'),
        approved_total_amount=('amount', lambda x: x[transactions.loc[x.index, 'state'] == 'approved'].sum())
    ).reset_index()
    return black
