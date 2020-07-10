pub mod records;
mod templates;

mod routes;
mod utils;

pub struct State {
    db: sqlx::sqlite::SqlitePool,
}

impl State {
    async fn new() -> tide::Result<Self> {
        let database_url = std::env::var("DATABASE_URL")?;
        let db = sqlx::sqlite::SqlitePool::new(&database_url).await?;
        Ok(Self { db })
    }
}

pub type Request = tide::Request<State>;

#[async_std::main]
async fn main() -> tide::Result<()> {
    tide::log::with_level(tide::log::LevelFilter::Info);

    let mut app = tide::with_state(State::new().await?);

    app.at("/welcome").get(routes::welcome);

    let mut articles = app.at("/articles");

    articles
        .post(routes::articles::create)
        .get(routes::articles::index);

    articles.at("/new").get(routes::articles::new);

    articles
        .at("/:article_id")
        .get(routes::articles::show)
        .delete(routes::articles::delete)
        .put(routes::articles::update);

    app.listen("127.0.0.1:8000").await?;
    Ok(())
}
