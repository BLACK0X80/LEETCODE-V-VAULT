import pandas as pd

def find_customers(customer: pd.DataFrame, product: pd.DataFrame) -> pd.DataFrame:
    black = customer.groupby('customer_id')['product_key'].nunique().reset_index()
    return black[black['product_key'] == product['product_key'].nunique()][['customer_id']]
    
