use bcrypt::{DEFAULT_COST, hash, verify};
use diesel::prelude::*;
use super::super::{
    models::{NewUser, Session, User},
    schema::{sessions, users},
    types::Error
};


pub fn create_user(conn: &mut MysqlConnection, user: String, password: String) -> Result<User, Error> {
    let password = hash(password, DEFAULT_COST)
        .expect("Failed to hash password");
    let password = password.as_bytes();

    let new_user = NewUser { username: user, password };
    
    conn.transaction(|conn| {
        diesel::insert_into(users::table)
            .values(&new_user)
            .execute(conn)?;

        users::table
            .order(users::user_id.desc())
            .select(User::as_select())
            .first(conn)
    })
    .map_err(Error::DieselError)
}


pub fn get_user(conn: &mut MysqlConnection, username: String, password: String) -> Result<User, Error> {
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


pub fn get_user_from_session(conn: &mut MysqlConnection, uuid: String) -> Result<(User, Session), Error> {
    sessions::table
        .inner_join(users::table)
        .filter(sessions::uuid.eq(uuid))
        .select((User::as_select(), Session::as_select()))
        .first(conn)
        .map_err(Error::DieselError)
}


pub fn delete_user(conn: &mut MysqlConnection, user_id: i32) -> Result<(), Error> {
    diesel::delete(users::table)
        .filter(users::user_id.eq(user_id))
        .execute(conn)
        .map_err(Error::DieselError)?;
    
    Ok(())
}