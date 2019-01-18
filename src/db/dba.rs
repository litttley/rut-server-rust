// db actor
use actix_web::actix::{Addr,SyncArbiter,Actor,SyncContext};
use diesel::prelude::PgConnection;
use diesel::r2d2::{ Pool, ConnectionManager };
use dotenv;
use num_cpus;

// This is db executor actor
pub struct Dba(pub Pool<ConnectionManager<PgConnection>>);

impl Actor for Dba {
    type Context = SyncContext<Self>;
}

pub fn init() -> Addr<Dba> {
    let db_url = dotenv::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let manager = ConnectionManager::<PgConnection>::new(db_url);
    let conn = Pool::builder().build(manager).expect("Failed to create pool.");
    SyncArbiter::start( num_cpus::get() * 1, move || { Dba(conn.clone()) })
}
