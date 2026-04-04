import pandas as pd

def find_employees(employee: pd.DataFrame) -> pd.DataFrame:
    black = employee.merge(employee, left_on='managerId', right_on='id', suffixes=('', '_mgr'))
    return black[black['salary'] > black['salary_mgr']][['name']].rename(columns={'name': 'Employee'})
