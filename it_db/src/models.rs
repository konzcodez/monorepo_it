use diesel::prelude::*;
use crate::schema::computers;
#[derive(Queryable, Selectable)]
#[diesel(table_name = computers)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Computer {
    pub id: i32,
    pub ip: String,
    pub name: String,
    pub os: String,
    pub snum: String,
    pub notes: String,
    pub model: String,
    pub manufacturer: String,
    pub cpu: String,
    pub memory: String,
    pub storage: String,
    pub install_date: String,
}
#[derive(Insertable)]
#[diesel(table_name = computers)]
pub struct NewComputer<'a> {
    pub ip : &'a str,
    pub name : &'a str,
}
