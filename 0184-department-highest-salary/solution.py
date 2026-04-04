import pandas as pd

def department_highest_salary(employee: pd.DataFrame, department: pd.DataFrame) -> pd.DataFrame:
    black = employee.merge(department, left_on='departmentId', right_on='id', suffixes=('', '_dept'))
    black['max_sal'] = black.groupby('departmentId')['salary'].transform('max')
    black = black[black['salary'] == black['max_sal']]
    return black.rename(columns={'name_dept': 'Department', 'name': 'Employee', 'salary': 'Salary'})[['Department', 'Employee', 'Salary']]
    
