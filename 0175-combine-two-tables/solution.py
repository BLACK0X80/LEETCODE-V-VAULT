import pandas as pd

def combine_two_tables(person: pd.DataFrame, address: pd.DataFrame) -> pd.DataFrame:
    black = person.merge(address, on='personId', how='left')
    return black[['firstName', 'lastName', 'city', 'state']]
