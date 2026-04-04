import pandas as pd

def find_investments(insurance: pd.DataFrame) -> pd.DataFrame:
    dup_tiv = insurance.groupby('tiv_2015')['tiv_2015'].transform('count') > 1
    unique_loc = insurance.groupby(['lat', 'lon'])['pid'].transform('count') == 1
    black = insurance[dup_tiv & unique_loc]
    return pd.DataFrame({'tiv_2016': [round(black['tiv_2016'].sum(), 2)]})
