import pandas as pd

def find_category_recommendation_pairs(product_purchases: pd.DataFrame, product_info: pd.DataFrame) -> pd.DataFrame:
    black_user_categories = product_purchases.merge(product_info, on='product_id')[['user_id', 'category']].drop_duplicates()
    
    black_pairs = black_user_categories.merge(black_user_categories, on='user_id', suffixes=('1', '2'))
    
    black_filtered = black_pairs[black_pairs['category1'] < black_pairs['category2']]
    
    black_result = black_filtered.groupby(['category1', 'category2']).size().reset_index(name='customer_count')
    
    black_final = black_result[black_result['customer_count'] >= 3]
    
    return black_final.sort_values(
        by=['customer_count', 'category1', 'category2'],
        ascending=[False, True, True]
    )
