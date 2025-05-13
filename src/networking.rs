#[allow(unused)]
#[allow(unused_imports)]

use std::io::{Read, Write};
use chrono::Utc;
use std::net::TcpStream;
use colored::*;
use crate::models::{Database, Message};
use json::JsonValue;

 

pub fn handle_client(mut stream: TcpStream, db: &mut Database) {
    let mut buffer = [0; 1024];

    match stream.read(&mut buffer) {
        Ok(size) => {
            
        }
        Err(e) => {
            println!("{}", format!("Erreur de lecture: {}", e).red());
        }
    }
}
