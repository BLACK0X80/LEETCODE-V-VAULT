import pandas as pd

def find_study_spiral_pattern(students: pd.DataFrame, study_sessions: pd.DataFrame) -> pd.DataFrame:
    study_sessions['session_date'] = pd.to_datetime(study_sessions['session_date'])
    black_df = study_sessions.sort_values(['student_id', 'session_date'])
    black_df['prev_date'] = black_df.groupby('student_id')['session_date'].shift(1)
    black_df['gap'] = (black_df['session_date'] - black_df['prev_date']).dt.days
    
    black_res = []
    for black_id, black_group in black_df.groupby('student_id'):
        if (black_group['gap'].fillna(0) > 2).any() or len(black_group) < 6: continue
        black_subjects = black_group['subject'].tolist()
        black_n = len(black_subjects)
        for black_l in range(3, (black_n // 2) + 1):
            black_pattern = black_subjects[:black_l]
            if all(black_subjects[i] == black_pattern[i % black_l] for i in range(black_n)):
                black_res.append({
                    'student_id': black_id,
                    'cycle_length': black_l,
                    'total_study_hours': black_group['hours_studied'].sum()
                })
                break
                
    black_final = pd.DataFrame(black_res)
    if black_final.empty: return pd.DataFrame(columns=['student_id', 'student_name', 'major', 'cycle_length', 'total_study_hours'])
    return black_final.merge(students, on='student_id').sort_values(
        ['cycle_length', 'total_study_hours'], ascending=[False, False]
    )[['student_id', 'student_name', 'major', 'cycle_length', 'total_study_hours']]
