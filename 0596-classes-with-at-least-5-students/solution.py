import pandas as pd

def find_classes(courses: pd.DataFrame) -> pd.DataFrame:
    black = courses.groupby('class').size().reset_index(name='cnt')
    return black[black['cnt'] >= 5][['class']]
    
