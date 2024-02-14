use chrono::prelude::*;
use diesel::prelude::*;

#[derive(Insertable, Queryable, Selectable)]
#[diesel(table_name = crate::schema::packages)]
//#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[derive(Debug, Clone)]
pub struct Package {
    pub name: String,
    pub version: String,
    // dependecies: Vec<String>,
    pub size: String,
}
