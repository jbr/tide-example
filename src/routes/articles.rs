use crate::records::{Article, PartialArticle};
use crate::templates::articles::*;
use crate::utils;
use sqlx::prelude::*;

pub async fn index(request: crate::Request) -> tide::Result {
    let articles = Article::all().fetch_all(&request.state().db).await?;
    Ok(IndexTemplate::for_articles(articles.as_slice()).into())
}

pub async fn show(request: crate::Request) -> tide::Result {
    let article = Article::find_by_id(request.param("article_id")?)
        .fetch_one(&request.state().db)
        .await?;

    Ok(ShowTemplate::for_article(&article).into())
}

pub async fn delete(request: crate::Request) -> tide::Result {
    Article::delete_by_id(request.param("article_id")?)
        .execute(&request.state().db)
        .await?;

    // if we had sessions, we'd set a flash message with whether this was successful
    Ok(tide::Redirect::new("/").into())
}

pub async fn update(mut request: crate::Request) -> tide::Result {
    let article: PartialArticle = utils::deserialize_body(&mut request).await?;
    let article_id = request.param("article_id")?;
    let rows_updated = article
        .update_by_id(article_id)
        .execute(&request.state().db)
        .await?;

    if rows_updated == 1 {
        Ok(tide::Redirect::new(format!("/articles/{}", article_id)).into())
    } else {
        Ok(ArticleForm::for_partial_article(&article).into())
    }
}

pub async fn create(mut request: crate::Request) -> tide::Result {
    let db = &request.state().db;
    let mut tx = db.begin().await?;
    let article: PartialArticle = utils::deserialize_body(&mut request).await?;
    let created = article.create().execute(&mut tx).await?;

    if created == 1 {
        let (last_id,) = Article::last_id().fetch_one(&mut tx).await?;
        Ok(tide::Redirect::new(format!("/articles/{}", last_id)).into())
    } else {
        Ok(ArticleForm::for_partial_article(&article).into())
    }
}

pub async fn new(_request: crate::Request) -> tide::Result {
    let article = PartialArticle::default();
    Ok(ArticleForm::for_partial_article(&article).into())
}
