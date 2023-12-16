use it_db::*; 
use diesel::prelude::*;
use self::models::Computer;


fn main() {

    use self::schema::computers::dsl::*;

    let mut connection = establish_connection();
    let results = computers
        .filter(name.eq("test"))
        .limit(5)
        .select(Computer::as_select())
        .load(&mut connection)
        .expect("Error loading computers");
    println!("Displaying {} computers", results.len());
    for computer in results {
        println!("{}", computer.name);
        println!("----------\n");
        println!("{}", computer.ip);
    }
}
