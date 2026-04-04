import pandas as pd

def movie_rating(movies: pd.DataFrame, users: pd.DataFrame, movie_rating: pd.DataFrame) -> pd.DataFrame:
    black1 = movie_rating.groupby('user_id').size().reset_index(name='cnt')
    black1 = black1.merge(users, on='user_id')
    black1 = black1.sort_values(['cnt', 'name'], ascending=[False, True])
    top_user = black1.iloc[0]['name']

    black2 = movie_rating[movie_rating['created_at'].dt.to_period('M') == '2020-02']
    black2 = black2.groupby('movie_id')['rating'].mean().reset_index()
    black2 = black2.merge(movies, on='movie_id')
    black2 = black2.sort_values(['rating', 'title'], ascending=[False, True])
    top_movie = black2.iloc[0]['title']

    return pd.DataFrame({'results': [top_user, top_movie]})
