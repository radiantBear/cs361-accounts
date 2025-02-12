pub mod db;
pub mod models;
pub mod schema;

use std::io::{stdin, stdout, Write};
use crate::db::*;

fn main() {
    println!("Establishing connection...");
    let connection = &mut establish_connection();
    println!("Connection established...");
    
    // Get inputs
    let (username, password) = get_creds();
    let username = username.trim();
    let password = password.trim();

    // Store user
    let user = create_user(connection, username, password);
    println!("\nSaved user {} at {}", user.username, user.date_created);


    // Get inputs
    let (username, password) = get_creds();
    let username = username.trim();
    let password = password.trim();

    // Check if user exists
    if let Some(user) = get_user(connection, username, password) {
        println!("Found user {} (ID {})", user.username, user.user_id);
    }
}


fn get_creds() -> (String, String) {
    let mut username = String::new();
    let mut password = String::new();

    print!("Username: ");
    stdout().flush().unwrap();
    stdin().read_line(&mut username).unwrap();

    print!("Password: ");
    stdout().flush().unwrap();
    stdin().read_line(&mut password).unwrap();

    (username, password)
}