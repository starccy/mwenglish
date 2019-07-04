use diesel::{r2d2, MysqlConnection};
use diesel::r2d2::ConnectionManager;

pub type Pool = r2d2::Pool<ConnectionManager<MysqlConnection>>;

pub fn establish_connection() -> Result<Pool, String> {
    dotenv::dotenv().ok();
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let manager = ConnectionManager::<MysqlConnection>::new(&database_url);
    let pool = r2d2::Pool::builder().build(manager).map_err(|e| e.to_string())?;
    Ok(pool)
}