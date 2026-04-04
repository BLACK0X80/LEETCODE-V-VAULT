import pandas as pd

def largest_orders(orders: pd.DataFrame) -> pd.DataFrame:
    black = orders.groupby('customer_number').size().idxmax()
    return pd.DataFrame({'customer_number': [black]})
    
