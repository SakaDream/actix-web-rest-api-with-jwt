use diesel::{
    pg::PgConnection,
    r2d2::{self, ConnectionManager},
};

embed_migrations!();

pub type Connection = PgConnection;
pub type Pool = r2d2::Pool<ConnectionManager<Connection>>;

pub fn migrate_and_config_db(url: &str) -> Pool {
    info!("Migrating and configurating database...");    
    let manager = ConnectionManager::<Connection>::new(url);
    let pool = r2d2::Pool::builder().build(manager).expect("Failed to create pool.");
    embedded_migrations::run(&pool.get().expect("Failed to migrate."));

    pool
}
