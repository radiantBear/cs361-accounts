use diesel::prelude::*;
use rand::{distr::Alphanumeric, Rng};
use super::super::{
    models::{NewSession, Session},
    schema::sessions,
    types::Error
};


pub fn create_session(conn: &mut MysqlConnection, user_id: i32) -> Result<Session, Error> {
    let uuid: String = rand::rng()
        .sample_iter(&Alphanumeric)
        .take(128)
        .map(char::from)
        .collect();

    let new_session = NewSession { uuid, user_id };

    conn.transaction(|conn| {
        diesel::insert_into(sessions::table)
            .values(&new_session)
            .execute(conn)?;
        
        sessions::table
            .order(sessions::session_id.desc())
            .select(Session::as_select())
            .first(conn)
    })
    .map_err(Error::DieselError)
}


pub fn get_session(conn: &mut MysqlConnection, uuid: String) -> Result<Session, Error> {
    sessions::table
        .filter(sessions::uuid.eq(uuid))
        .first::<Session>(conn)
        .map_err(Error::DieselError)
}

