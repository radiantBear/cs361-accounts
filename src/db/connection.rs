use diesel::prelude::*;

use crate::config::CONFIG;


pub fn establish() -> Result<MysqlConnection, ConnectionError> {    
    MysqlConnection::establish(
        CONFIG.get().unwrap().database_url.as_str()
    )
}