import pandas as pd

def sales_analysis(sales: pd.DataFrame) -> pd.DataFrame:
    black = sales.copy()
    black['first_year'] = black.groupby('product_id')['year'].transform('min')
    black = black[black['year'] == black['first_year']]
    return black[['product_id', 'first_year', 'quantity', 'price']]
    
