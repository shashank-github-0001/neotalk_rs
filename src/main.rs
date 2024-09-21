mod apis;
mod crud;
mod models;

#[derive(Clone)]
pub struct AppState {
    pub pool: sqlx::Pool<sqlx::Postgres>,
}

#[async_std::main]
async fn main() -> Result<(), sqlx::Error> {
    dotenv::dotenv().ok();
    let url = std::env::var("DATABASE_URL").expect("Database url not set");
    let pool = sqlx::postgres::PgPoolOptions::new()
        .max_connections(5)
        .connect(&url.to_string())
        .await?;
    let mut app = tide::with_state(AppState { pool: pool.clone() });
    app.at("/").get(apis::get_user_list);
    app.at("/create").post(apis::post_user);
    app.listen("0.0.0.0:8000").await?;
    return Ok(());
}
