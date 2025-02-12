use diesel::prelude::*;
use rand::{distr::Alphanumeric, Rng};
use super::super::{
    models::{NewSession, Session},
    schema::sessions,
    types::Error
};


pub fn create_session(conn: &mut MysqlConnection, user_id: i32) -> Result<(), Error> {
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
    sessions::table
        .filter(sessions::uuid.eq(uuid))
        .first::<Session>(conn)
        .map_err(Error::DieselError)
}

