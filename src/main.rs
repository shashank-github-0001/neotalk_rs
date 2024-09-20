mod models;
mod crud;

#[tokio::main]
async fn main() -> Result<(), sqlx::Error>{
    dotenv::dotenv().ok();
    let url = std::env::var("DATABASE_URL").expect("Database url not set");
    println!("Database url: {}", url);
    let _pool = sqlx::postgres::PgPoolOptions::new().max_connections(5).connect(&url.to_string()).await?;
    return Ok(());
}
