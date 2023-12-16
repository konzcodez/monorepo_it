use it_db::*;
use diesel::PgConnection;
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

fn main() {
    let mut connection = &mut establish_connection();
    //take command line arguments
    let args: Vec<String> = std::env::args().collect();
    let command = &args[1];
    if command == "help" {
        help();
    }
    if command =="all" {
        list_all_computers(&mut connection);
    }
    if command == "add" {
        //add_computer(&mut connection);
    }
    if command == "search" {
       search_computer(&mut connection); 
    }
    if command == "change" {
       change_computer(&mut connection); 
    }
    if command == "remove" {
        remove_computer(&mut connection); 
    }
}
