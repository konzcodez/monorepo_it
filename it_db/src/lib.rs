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

pub fn add_computer(conn: &mut PgConnection, name: &str, ip: &str, os: &str, snum: &str, notes: &str, model: &str, manufacturer: &str, cpu: &str, memory: &str, storage: &str, installdate: &str ) -> Computer {
    use crate::schema::computers;

    let new_computer = NewComputer {
        name,
        ip,
        os,
        snum,
        notes,
        model,
        manufacturer,
        cpu,
        memory,
        storage,
        installdate,
    };

    diesel::insert_into(computers::table)
        .values(&new_computer)
        .returning(Computer::as_returning())
        .get_result(conn)
        .expect("Error saving new computer")
}
/* Commdands */
pub fn list_all_computers(conn: &mut PgConnection) {
    let results = schema::computers::table
        .load::<Computer>(conn)
        .expect("Error loading computers");
    println!("Displaying {} computers", results.len());
    for computer in results {
        println!("{}", computer.ip);
        println!("----------\n");
        println!("Name: {}", computer.name);
        println!("OS: {}", computer.os);
        println!("Serial Number: {}", computer.snum);
        println!("Notes: {}", computer.notes);
        println!("Model: {}", computer.model);
        println!("Manufacturer: {}", computer.manufacturer);
        println!("CPU: {}", computer.cpu);
        println!("Memory: {}", computer.memory);
        println!("Storage: {}", computer.storage);
        println!("Install Date: {}", computer.installdate);
    }

}
pub fn help() {
}
pub fn remove_computer(conn: &mut PgConnection, name: &str, ip: &str) {
}
pub fn update_computer(conn: &mut PgConnection, change_what: &str, change_to: &str, name: &str, ip: &str) {
}
pub fn search_computer(conn: &mut PgConnection, name: &str, ip: &str) {
}
