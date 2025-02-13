use diesel::{prelude::*, result::Error as DieselError};
use crate::utils::rand;
use super::super::{
    models::{NewCsrfToken, CsrfToken},
    schema::csrf_tokens,
    types::Error
};


pub fn create_csrf_token(conn: &mut MysqlConnection) -> Result<CsrfToken, Error> {
    let uuid = rand::generate_alphanumeric(128);

    let new_session = NewCsrfToken { uuid };

    conn.transaction(|conn| {
        diesel::insert_into(csrf_tokens::table)
            .values(&new_session)
            .execute(conn)?;
        
        csrf_tokens::table
            .order(csrf_tokens::csrf_token_id.desc())
            .select(CsrfToken::as_select())
            .first(conn)
    })
    .map_err(Error::DieselError)
}


pub fn validate_csrf_token(conn: &mut MysqlConnection, csrf_token: String) -> Result<bool, Error> {
    let result = conn.transaction(|conn| {
        csrf_tokens::table
            .filter(csrf_tokens::uuid.eq(&csrf_token))
            .first::<CsrfToken>(conn)?;
        
        diesel::delete(csrf_tokens::table
            .filter(csrf_tokens::uuid.eq(csrf_token))
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
