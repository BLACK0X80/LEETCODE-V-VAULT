import pandas as pd

def top_three_salaries(employee: pd.DataFrame, department: pd.DataFrame) -> pd.DataFrame:
    def top3(group):
        return group[group['salary'].rank(method='dense', ascending=False) <= 3]
    
    result = employee.groupby('departmentId', group_keys=False).apply(top3).reset_index(drop=True)
    result = result.merge(department, left_on='departmentId', right_on='id')
    return result[['name_y', 'name_x', 'salary']].rename(columns={'name_y': 'Department', 'name_x': 'Employee'})
