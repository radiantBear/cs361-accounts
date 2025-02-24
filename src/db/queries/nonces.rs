use diesel::{prelude::*, result::Error as DieselError};
use crate::utils::rand;
use super::super::{
    models::{NewNonce, Nonce},
    schema::nonces,
    types::Error
};


pub fn create_nonce(conn: &mut MysqlConnection) -> Result<Nonce, Error> {
    let uuid = rand::generate_alphanumeric(128);

    let new_nonce = NewNonce { uuid };

    conn.transaction(|conn| {
        diesel::insert_into(nonces::table)
            .values(&new_nonce)
            .execute(conn)?;
        
        nonces::table
            .order(nonces::nonce_id.desc())
            .select(Nonce::as_select())
            .first(conn)
    })
    .map_err(Error::DieselError)
}


pub fn validate_nonce(conn: &mut MysqlConnection, nonce: String) -> Result<bool, Error> {
    let result = conn.transaction(|conn| {
        nonces::table
            .filter(nonces::uuid.eq(&nonce))
            .first::<Nonce>(conn)?;
        
        diesel::delete(nonces::table
            .filter(nonces::uuid.eq(nonce))
        ).execute(conn)
    });
    
    if let Err(DieselError::NotFound) = result {
        return Ok(false);
    }
    else if let Err(e) = result {
        return Err(Error::DieselError(e));
    }

    Ok(true)
}
