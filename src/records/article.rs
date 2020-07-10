use serde::{Deserialize, Serialize};
type Query = sqlx::Query<'static, sqlx::Sqlite>;
type QueryAs<T> = sqlx::QueryAs<'static, sqlx::Sqlite, T>;

#[derive(sqlx::FromRow, Debug, Deserialize, Serialize)]
pub struct Article {
    pub id: i64,
    pub text: String,
    pub title: String,
    created: i32,
    updated: i32,
}

impl crate::utils::AsRoute for Article {
    fn as_route(&self) -> std::borrow::Cow<str> {
        format!("/articles/{}", self.id).into()
    }
}

#[derive(Debug, Deserialize, Serialize, Default)]
pub struct PartialArticle {
    pub text: Option<String>,
    pub title: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, Default)]
pub struct ArticleValidation {
    pub text_errors: Vec<String>,
    pub title_errors: Vec<String>,
}

impl PartialArticle {
    pub fn update_by_id(&self, id: i64) -> Query {
        sqlx::query(
            "UPDATE articles (text, title, updated) VALUES (
            COALESCE($1, articles.text),
            COALESCE($2, articles.title),
            datetime('now')
          ) WHERE id = $3",
        )
        .bind(&self.text)
        .bind(&self.title)
        .bind(id)
    }

    pub fn create(&self) -> Query {
        sqlx::query(
            "INSERT INTO articles (text, title, created, updated) VALUES (
            $1, $2, DATETIME('now'), DATETIME('now')
          )",
        )
        .bind(&self.text)
        .bind(&self.title)
    }
}

impl Article {
    pub fn all() -> QueryAs<Self> {
        sqlx::query_as("SELECT * FROM articles")
    }

    pub fn last_id() -> QueryAs<(i64,)> {
        sqlx::query_as("SELECT last_insert_rowid()")
    }

    pub fn find_by_id(id: i64) -> QueryAs<Self> {
        sqlx::query_as("SELECT * FROM articles WHERE id = ?").bind(id)
    }

    pub fn delete_by_id(id: i64) -> Query {
        sqlx::query("DELETE FROM articles WHERE id = ?").bind(id)
    }

    pub fn update(&self, partial: PartialArticle) -> Query {
        partial.update_by_id(self.id)
    }
}
