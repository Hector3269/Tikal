mod model;

use model::User;
use tikal::prelude::*;

fn main() {
    let sql_create = User::generate_create_table_sql("sqlite");
    println!("Create Table SQL:\n{}", sql_create);
   
    println!("User table: {}", User::table_name());

    println!("User primary key: {}", User::primary_key());
}
