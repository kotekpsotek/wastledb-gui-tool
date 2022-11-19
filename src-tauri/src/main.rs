#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

mod payloads;

use tauri::Manager;
use payloads::EstablishConnection;
use serde_json::{ self, json };
use std::{
    sync::{Arc, Mutex},
    net::TcpStream, io::{Write, Read}
};

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

fn event_listeners(app: &mut tauri::App) {
    let app_handler = Arc::new(app.handle());
    let server_url = Arc::new(Mutex::new(String::new()));
    let session_id_storage = Arc::new(Mutex::new(String::new()));

    // Establish connection with dbs (Calling always as 1 event in event tree)
    app.listen_global("establish-connection", {
        let app_handler = Arc::clone(&app_handler); // server ip address without "database" and protocol name so i.e: 127.0.0.1:20050
        let session_id_storage = Arc::clone(&session_id_storage);
        let server_url_storage = server_url.clone();
        move |event| {
            let EstablishConnection { server_url, user_name, user_password, db_name, create_encrypted_connection, rsapublic_key } = serde_json::from_str::<EstablishConnection>(event.payload().unwrap()).expect("Couldn't deserialize message");

            if server_url.len() > 0 && user_name.len() > 0 && user_password.len() > 0 {
                let url_correcteness = || {
                    if server_url.starts_with("wastledb://") && server_url.replace("wastledb://", "").split("/").collect::<Vec<_>>()[0].contains(":") {
                        true
                    }
                    else {
                        false
                    }
                };

                if url_correcteness() {
                    let url_address = server_url.replace("wastledb://", "");
                    let url_address = url_address.split("/").collect::<Vec<_>>();
                    
                    match TcpStream::connect(url_address[0]) {
                        Ok(mut stream) => {
                            // Assign address to dbs after successfull captured connection
                            *server_url_storage.lock().unwrap() = url_address[0].to_string();

                            // ...
                            let database_name = if url_address.len() > 1 {
                                Some(url_address[1])
                            }
                            else if db_name.is_some() {
                                Some(db_name.as_ref().unwrap().as_str())
                            }
                            else {
                                None
                            };

                            // Reqister. Request
                            let mut register_cmd = format!("Register;login|x=x|{login} 1-1 password|x=x|{password}", login = user_name, password = user_password);
                            if let Some(database_name) = database_name {
                                register_cmd.push_str(&format!(" 1-1 connect_auto|x=x|{}", database_name));
                            }
                            stream.write(register_cmd.as_bytes()).expect("Couldn't register user");

                            // Register. Response
                            let response: &mut [u8; 16384] = &mut [0; 16 * 1024];
                            stream.read(response).expect("Couldn't read response from server");
                            let response_string = String::from_utf8(response.to_vec()).expect("Couldn't convert response to string");
                            let response_slices = response_string.split(";").collect::<Vec<_>>();

                            if response_slices.len() > 0 {
                                let type_pointer = response_slices[0].to_lowercase();
                                match type_pointer.as_str() {
                                    "ok" => {
                                        // Achive session id
                                        let session_id = response_slices[1];
                                        *session_id_storage.lock().unwrap() = session_id.to_string(); // save session into session "global storage"

                                        // Everything is ok
                                        // TODO: Whether user is connected to database must be attached in dbs response because also it is a little likely to attached database doesn't exists
                                        let connected_with_database = {
                                            if database_name.is_some() { // indicate frontend whether it is now connected with database to afford to call as next adequate events
                                                true
                                            }
                                            else {
                                                false
                                            }
                                        };
                                        app_handler.emit_all::<Option<String>>("connection-acquired", Some(json!({ "connected_with_database": connected_with_database, "database_name": database_name }).to_string())).expect("Couldn't emit event")
                                    },
                                    "err" => {
                                        // Error recived
                                        if response_slices.len() > 1 {
                                            let reason = response_slices[1].trim();
                                            app_handler.emit_all("couldnt-establish-connection", format!("couldn't connect from this reason:{}", reason)).expect("Couldn't emit event")
                                        }
                                        else {   
                                            app_handler.emit_all("couldnt-establish-connection", "unsupported response").expect("Couldn't emit event")
                                        }
                                    },
                                    "inclogin" => app_handler.emit_all("couldnt-establish-connection", "incorrect login").expect("Couldn't emit event"),
                                    _ => app_handler.emit_all("couldnt-establish-connection", "unsupported response").expect("Couldn't emit event")
                                }
                            }
                            else {
                                app_handler.emit_all("couldnt-establish-connection", "unsupported response").expect("Couldn't emit event")   
                            }
                        },
                        Err(_) => app_handler.emit_all("couldnt-establish-connection", "couldnt connect with dbs").expect("Couldn't emit event")
                    }       
                }
                else {
                    app_handler.emit_all("couldnt-establish-connection", "connection url is incorrect")
                        .expect("Couldn't convert response to string");
                }
            }
            else {
                app_handler.emit_all("couldnt-establish-connection", "you must attach login options")
                    .expect("Couldn't emit event");
            }
        }
    });

    // TODO: Renundant code below (Simplify that to one function)
    // Listen to show user tables list
    app.listen_global("show-tables", {
        let app_handler = app_handler.clone();
        let session_id_storage = session_id_storage.clone();
        let server_url_storage = server_url.clone();
        move |_| {
            let session_id = session_id_storage
                .lock()
                .unwrap()
                .to_owned();

            let command = format!("Show;what|x=x|database_tables 1-1 unit_name|x=x|none 1-1 session_id|x=x|{}", session_id);

            // Connection
            let mut tcp = TcpStream::connect(server_url_storage.lock().unwrap().to_owned()).expect("Couldn't establish connection with dbs"); 
                // ...Request
            tcp.write(command.as_bytes()).expect("Couldn't send request");
                // ...Response
            let response_bytes: &mut [u8; 1024 * 16] = &mut [0; 16384];
            tcp.read(response_bytes).expect("Couldn't read response");
            let response_string = String::from_utf8(response_bytes.to_vec()).expect("Couldn't convert response to UTF-8 string").replace("\0", "");
            let resp_s = response_string.split(";").collect::<Vec<_>>();

            // Rebound to frontend
            if resp_s[0].to_lowercase() == "ok" {
                // Send tables to frontend
                app_handler.emit_all("show-tables-res", resp_s[1]).expect("Couldn't emit event"); // Send to frontend JSON object recived from database
            }
            else {
                // Emitt that error
                app_handler.emit_all("error", "Couldn't achive tables list from dbs").expect("Couldn't emit event");
            }
        }
    });

    // Download from "dbs" and send to "frontend" databases list
    app.listen_global("show-databases", {
        let app_handler = app_handler.clone();
        let session_id_storage = session_id_storage.clone();
        let server_url_storage = server_url.clone();
    
        move |_| {
            let session_id = session_id_storage
                .lock()
                .unwrap()
                .to_owned();

            let command = format!("Show;what|x=x|databases 1-1 unit_name|x=x|none 1-1 session_id|x=x|{}", session_id);

            // Connection
            let mut tcp = TcpStream::connect(server_url_storage.lock().unwrap().to_owned()).expect("Couldn't establish connection with dbs"); 
                // ...Request
            tcp.write(command.as_bytes()).expect("Couldn't send request");
                // ...Response
            let response_bytes: &mut [u8; 1024 * 16] = &mut [0; 16384];
            tcp.read(response_bytes).expect("Couldn't read response");
            let response_string = String::from_utf8(response_bytes.to_vec()).expect("Couldn't convert response to UTF-8 string").replace("\0", "");
            let resp_s = response_string.split(";").collect::<Vec<_>>();

            // Rebound to frontend
            if resp_s[0].to_lowercase() == "ok" {
                // Send databases to frontend
                app_handler.emit_all("show-databases-res", resp_s[1]).expect("Couldn't emit event"); // Send to frontend JSON object recived from database
            }
            else {
                // Emitt that error
                app_handler.emit_all("error", "Couldn't achive list of databases from dbs").expect("Couldn't emit event");
            }
        }
    });
}

fn main() {
    tauri::Builder::default()
        .setup(|app| {
            // Listen Events
            event_listeners(app);

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
