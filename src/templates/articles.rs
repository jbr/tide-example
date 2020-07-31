use crate::records::{Article, PartialArticle};
use askama::Template;

#[derive(Template)]
#[template(path = "articles/form.html")]
pub struct ArticleForm<'a> {
    title: &'a str,
    text: &'a str,
    action: String,
}

impl<'a> ArticleForm<'a> {
    pub fn for_partial_article(article: &'a PartialArticle) -> Self {
        Self {
            title: article.title.as_deref().unwrap_or_default(),
            text: article.text.as_deref().unwrap_or_default(),
            action: "/articles".into(),
        }
    }

    // pub fn for_article(article: &'a Article) -> Self {
    //     Self {
    //         title: &article.title,
    //         text: &article.text,
    //         action: format!("/articles/{}", article.id),
    //     }
    // }
}

#[derive(Template)]
#[template(path = "articles/index.html")]
pub struct IndexTemplate<'a> {
    articles: &'a [Article],
}

impl<'a> IndexTemplate<'a> {
    pub fn for_articles(articles: &'a [Article]) -> Self {
        Self { articles }
    }
}

#[derive(Template)]
#[template(path = "articles/show.html")]
pub struct ShowTemplate<'a> {
    title: &'a str,
    text: &'a str,
}

impl<'a> ShowTemplate<'a> {
    pub fn for_article(article: &'a Article) -> Self {
        Self {
            title: &article.title,
            text: &article.text,
        }
    }
}
