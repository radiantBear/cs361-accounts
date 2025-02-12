use bcrypt::{DEFAULT_COST, hash, verify};
use diesel::prelude::*;
use super::super::{
    models::{NewUser, User},
    schema::{sessions, users},
    types::Error
};


pub fn create_user(conn: &mut MysqlConnection, user: &str, password: &str) -> Result<(), Error> {
    let password = hash(password, DEFAULT_COST)
        .expect("Failed to hash password");
    let password = password.as_bytes();

    let new_user = NewUser { username: user, password };
    
    diesel::insert_into(users::table)
        .values(&new_user)
        .execute(conn)
        .map_err(Error::DieselError)?;
    
    Ok(())
}


pub fn get_user(conn: &mut MysqlConnection, username: &str, password: &str) -> Result<User, Error> {
    let user = users::table
        .filter(users::username.eq(username))
        .first::<User>(conn)
        .map_err(Error::DieselError)?;
    
    let hashed_password = std::str::from_utf8(&user.password)
        .map_err(|_| Error::CustomError(String::from("Hashed string included invalid UTF-8 characters")))?;

    let hashes_match = verify(password, hashed_password)
        .map_err(|_| Error::CustomError(String::from("Error checking passwords")))?;
    
    if hashes_match {
        Ok(user)
    } else {
        Err(Error::CustomError(String::from("Incorrect password")))
    }
}


pub fn get_user_from_session(conn: &mut MysqlConnection, uuid: &str) -> Result<User, Error> {
    sessions::table
        .inner_join(users::table)
        .filter(sessions::uuid.eq(uuid))
        .select(User::as_select())
        .first(conn)
        .map_err(Error::DieselError)
}