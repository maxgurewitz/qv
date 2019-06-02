use diesel::pg::PgConnection;
use diesel::r2d2::{ConnectionManager, Pool};
use dotenv::dotenv;
use std::env;

pub type PgPool = Pool<ConnectionManager<PgConnection>>;

pub fn build_pg_pool() -> PgPool {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pg_manager = ConnectionManager::<PgConnection>::new(database_url);
    Pool::builder().max_size(2).build(pg_manager).unwrap()
}
