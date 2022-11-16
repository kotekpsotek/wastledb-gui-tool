#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

mod payloads;

use tauri::Manager;
use payloads::EstablishConnection;
use serde_json;
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
    let session_id_storage = Arc::new(Mutex::new(String::new()));

    app.listen_global("establish-connection", {
        let app_handler = Arc::clone(&app_handler);
        let session_id_storage = Arc::clone(&session_id_storage);
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
                                        app_handler.emit_all::<Option<String>>("connection-acquired", None).expect("Couldn't emit event")
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
