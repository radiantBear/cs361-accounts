use std::env;
use diesel::prelude::*;
use diesel::result::Error as DieselError;
use dotenvy::dotenv;
use rand::{distr::Alphanumeric, Rng};
use crate::models::{User, Session};


pub enum Error {
    DieselError(DieselError),
    CustomError(String)
}


pub fn establish_connection() -> Result<MysqlConnection, ConnectionError> {
    dotenv().ok();
    
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    
    MysqlConnection::establish(&database_url)
}


pub fn create_user(conn: &mut MysqlConnection, user: &str, password: &str) -> Result<(), Error> {
    use crate::schema::users;
    use crate::models::NewUser;
    use bcrypt::{DEFAULT_COST, hash};


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
    use crate::schema::users;
    use bcrypt::verify;

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


pub fn create_session(conn: &mut MysqlConnection, user_id: i32) -> Result<(), Error> {
    use crate::schema::sessions;
    use crate::models::NewSession;

    let uuid: String = rand::rng()
        .sample_iter(&Alphanumeric)
        .take(128)
        .map(char::from)
        .collect();

    let new_session = NewSession { uuid, user_id };

    diesel::insert_into(sessions::table)
        .values(&new_session)
        .execute(conn)
        .map_err(Error::DieselError)?;


    Ok(())
}


pub fn get_session(conn: &mut MysqlConnection, uuid: &str) -> Result<Session, Error> {
    use crate::schema::sessions;

    Ok(
        sessions::table
            .filter(sessions::uuid.eq(uuid))
            .first::<Session>(conn)
            .map_err(Error::DieselError)?
    )
}


pub fn get_user_from_session(conn: &mut MysqlConnection, uuid: &str) -> Result<User, Error> {
    use crate::schema::{sessions, users};

    Ok(
        sessions::table
            .inner_join(users::table)
            .filter(sessions::uuid.eq(uuid))
            .select(User::as_select())
            .first(conn)
            .map_err(Error::DieselError)?
    )
}