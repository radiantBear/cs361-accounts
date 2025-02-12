use std::env;
use diesel::prelude::*;
use dotenvy::dotenv;
use crate::models::User;


pub fn establish_connection() -> MysqlConnection {
    println!("Starting...");
    dotenv().ok();
    
    println!("Getting database url...");
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    
    println!("Establishing connection to {}...", database_url);
    MysqlConnection::establish(&database_url)
        .unwrap_or_else(|e| panic!("Error connecting to {}: {:?}", database_url, e))
}


pub fn create_user(conn: &mut MysqlConnection, user: &str, password: &str) -> User {
    use crate::schema::users;
    use crate::models::NewUser;
    use bcrypt::{DEFAULT_COST, hash};


    let password = hash(password, DEFAULT_COST)
        .expect("Failed to hash password");
    let password = password.as_bytes();

    let new_user = NewUser { username: user, password };

    conn.transaction(|conn| {
        diesel::insert_into(users::table)
            .values(&new_user)
            .execute(conn)?;

        users::table
            .select(User::as_select())
            .first(conn)
    })
    .expect("Error saving user")
}

pub fn get_user(conn: &mut MysqlConnection, username: &str, password: &str) -> Option<User> {
    use crate::schema::users::dsl::{users, username as db_username};
    use bcrypt::verify;

    let user = users
        .filter(db_username.eq(username))
        .first::<User>(conn)
        .expect("Error fetching user");
    
    if verify(password, std::str::from_utf8(&user.password).unwrap()).expect("Error checking passwords") {
        return Some(user);
    }
    None
}