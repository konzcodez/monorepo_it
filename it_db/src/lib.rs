pub mod models;
pub mod schema;
use crate::schema::computers::dsl::*;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use dotenv::dotenv;
use std::env;
enum TypeOfSearch {
    PCNAME,
    IP,
    OS,
    SERIALNUMBER,
    MODEL,
    MANUFACTURER,
    CPU,
    MEMORY,
    STORAGE,
    INSTALLDATE,
}
pub fn establish_connection() -> PgConnection {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

use self::models::{Computer};

pub fn add_computer(conn: &mut PgConnection) {}
/* Commdands */
pub fn list_all_computers(conn: &mut PgConnection) {
    println!("Listing all computers!");
    let results = computers
        .select(Computer::as_select())
        .load(conn)
        .expect("Error loading computers");
    println!("Displaying {} computers", results.len());
    for computer in results {
        println!("--------------------------------");

        println!("PCNAME: {}", computer.name);
        println!("IP: {}", computer.ip);
        println!("OS: {}", computer.os);
        println!("SERIAL NUMBER: {}", computer.snum);
        println!("MODEL: {}", computer.model);
        println!("MANUFACTURER: {}", computer.manufacturer);
        println!("CPU: {}", computer.cpu);
        println!("MEMORY: {}", computer.memory);
        println!("STORAGE: {}", computer.storage);
        println!("INSTALLDATE: {}", computer.installdate);
    }
}
pub fn help() {
    /* Example of command line usage */
    /* pc-db add test-pc
     * this command will add a computer and prompt for all database fields
     * pc-db search -pcname test-pc
     * this command will search for a computer with the name test-pc
     * pc-db search -ip 198.0.10.24
     * pc-db change -pcname test-pc production-pc
     * changes name of test-pc to production-pc will prompt if there are multiple results
     * pc-db remove -pcname production-pc will prompt if there are multiple results
     */
    println!("Commands:");
    println!("pc-db add                                  ----THIS ADDS A COMPUTER");
    println!("pc-db all                                  ----THIS LISTS ALL COMPUTERS"); //lists all computers
    println!(
        "pc-db search -pcname <pcname>              ----THIS SEARCHES FOR A COMPUTER BY PCNAME"
    );
    println!("pc-db search -ip <ip> -pcname <pcname>     ----THIS SEARCHES FOR A COMPUTER BY IP ADDRESS, and PCNAME");
    println!(
        "pc-db change -pcname <pcname> <new-pcname> ----THIS CHANGES THE PCNAME OF A COMPUTER"
    );
    println!("pc-db change -ip <ip> <new-ip>             ----THIS CHANGES THE IP OF A COMPUTER");
    println!("pc-db remove -pcname <pcname>              ----THIS REMOVES A COMPUTER BY PCNAME");
    println!("What can I SEARCH by, and CHANGE you ask? PCNAME, IP, OS, SERIAL NUMBER, MODEL, MANUFACTURER, CPU, MEMORY, STORAGE, INSTALLDATE");
}
fn filter_type_cmd_menu() -> TypeOfSearch {
    let mut type_of_search = TypeOfSearch::PCNAME;
    println!("What would you like to search by?");
    println!("1. PCNAME");
    println!("2. IP");
    println!("3. OS");
    println!("4. SERIAL NUMBER");
    println!("5. MODEL");
    println!("6. MANUFACTURER");
    println!("7. CPU");
    println!("8. MEMORY");
    println!("9. STORAGE");
    println!("10. INSTALLDATE");
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    input = input.trim().to_string();
    match input.trim().parse::<u32>() {
        Ok(1) => type_of_search = TypeOfSearch::PCNAME,
        Ok(2) => type_of_search = TypeOfSearch::IP,
        Ok(3) => type_of_search = TypeOfSearch::OS,
        Ok(4) => type_of_search = TypeOfSearch::SERIALNUMBER,
        Ok(5) => type_of_search = TypeOfSearch::MODEL,
        Ok(6) => type_of_search = TypeOfSearch::MANUFACTURER,
        Ok(7) => type_of_search = TypeOfSearch::CPU,
        Ok(8) => type_of_search = TypeOfSearch::MEMORY,
        Ok(9) => type_of_search = TypeOfSearch::STORAGE,
        Ok(10) => type_of_search = TypeOfSearch::INSTALLDATE,
        _ => println!("Invalid input"),
    }
    type_of_search
}

pub fn search_computer(conn: &mut PgConnection, args: Vec<String>) {
}

pub fn remove_computer(conn: &mut PgConnection, args: Vec<String>) {}
pub fn update_computer(conn: &mut PgConnection, args: Vec<String>) {}
