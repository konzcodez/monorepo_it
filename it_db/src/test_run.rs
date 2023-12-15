use it_db::*; 
use diesel::prelude::*;



fn main() {
    use self::schema::computers::dsl::*;

    let connection = establish_connection();
    let results = computers
        .filter(name.eq("test"))
        .limit(5)
        .select(Computer::as_select())
        .load(connection)
        .expect("Error loading computers");
    println!("Displaying {} computers", results.len());
    for computer in results {
        println!("{}", computer.name);
        println!("----------\n");
        println!("{}", computer.ip);
    }
}

fn create_computer() -> Computer {
    use self::schema::computers;
    
    let new_computer = NewComputer {name = "pc-test" ,ip = "135.101.1.81"};

    diesel::insert_into(computers::table)
        .values(&new_computer)
        .get_result(&connection)
        .expect("Error saving new computer")
}
