pub mod models;
pub mod schema;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use dotenv::dotenv;
use std::env;
pub fn establish_connection() -> PgConnection {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

use self::models::{NewComputer, Computer};
pub fn new_computer(conn: &mut PgConnection, name: &str, ip: &str) -> Computer {
    use crate::schema::computers;

    let new_computer = NewComputer {
        name: name,
        ip: ip,
        os: "",
        snum: "",
        notes: "",
        model: "",
        manufacturer: "",
        cpu: "",
        memory: "",
        storage: "",
        installdate: "",
    };

    diesel::insert_into(computers::table)
        .values(&new_computer)
        .returning(Computer::as_returning())
        .get_result(conn)
        .expect("Error saving new computer")
}
