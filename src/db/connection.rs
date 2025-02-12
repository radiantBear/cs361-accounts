use std::env;
use diesel::prelude::*;
use dotenvy::dotenv;


pub fn establish() -> Result<MysqlConnection, ConnectionError> {
    dotenv().ok();
    
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    
    MysqlConnection::establish(&database_url)
}