use std::net::TcpListener;
use crate::models::Database;
use crate::networking::handle_client;
mod models;
mod networking;


fn main(){
    let mut db = Database {
        messages : vec![],
        history : vec![]
    };
   let listener = TcpListener::bind("127.0.0.1:8080").unwrap();
    for stream in listener.incoming() {
        match stream { 
            Ok(stream) => {
                handle_client(stream,&mut db); 
            }
            Err(e) => {
                println!("Erreur de connexion: {}", e);
            }
        }
    }
}

