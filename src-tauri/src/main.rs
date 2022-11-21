#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

mod payloads;

use tauri::Manager;
use payloads::{EstablishConnection, ExecuteSqlQuery};
use serde_json::{ self, json };
use std::{
    sync::{Arc, Mutex},
    net::TcpStream, io::{Write, Read},
    thread
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

    // Call keep-alice command from time to time // Call Keep-Alive when user has been login for every 10 seconds
    // Spawn new not-blocking thread
    thread::spawn({
        let session_id = session_id_storage.clone();
        let server_url = server_url.clone();
        let app_handler = app_handler.clone();
        
        move || {
            loop {
                let session_id = session_id.lock().unwrap().to_owned();
                
                thread::sleep(std::time::Duration::from_secs(10));

                if session_id.len() > 0 {
                    match TcpStream::connect(server_url.lock().unwrap().to_owned()) {
                        Ok(mut stream) => {
                            // Request
                            let command = format!("Keep-Alive;{}", session_id);
                            stream.write(command.as_bytes()).expect("Couldn't call Keep-Alive command");

                            // Response
                            let buf: &mut [u8; 1024 * 16] = &mut [0;16384];
                            match stream.read(buf) {
                                Ok(_) => {
                                    let resp_str = String::from_utf8(buf.to_vec())
                                        .expect("Couldn't convert response to UTF-8 string")
                                        .replace("\0", "");
    
                                    // Process response
                                    let resp_str_sli = resp_str.split(";")
                                        .collect::<Vec<_>>();
                                    
                                    match resp_str_sli[0].to_lowercase().as_str() {
                                        "ok" => (),
                                        "err" => {
                                            // Emit error to client when error has been detected
                                            let reason_err = {
                                                if resp_str_sli.len() > 1 {
                                                    resp_str_sli[1]
                                                }
                                                else {
                                                    "Couldn't update session live-time"
                                                }
                                            };
                                            app_handler.emit_all("error", reason_err)
                                                .expect("Couldn't emit event");
                                        },
                                        _ => ()
                                    }
                                },
                                Err(_) => ()
                            };
                        },
                        Err(_) => ()
                    };
                }
            }
        }
    });

    // Establish connection with dbs (Calling always as 1 event in event tree)
    app.listen_global("establish-connection", {
        let app_handler = Arc::clone(&app_handler); // server ip address without "database" and protocol name so i.e: 127.0.0.1:20050
        let session_id_storage = Arc::clone(&session_id_storage);
        let server_url_storage = server_url.clone();
        move |event| {
            let EstablishConnection { server_url, user_name, user_password, db_name, create_encrypted_connection: _, rsapublic_key: _ } = serde_json::from_str::<EstablishConnection>(event.payload().unwrap()).expect("Couldn't deserialize message");

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

    // Handle and parse request to DBS
    fn handle_dbs_request(command: String, server_url: String) -> Result<Vec<String>, ()> {
        // Connection
        let mut tcp = TcpStream::connect(server_url).expect("Couldn't establish connection with dbs"); 
            // ...Request
        tcp.write(command.as_bytes()).expect("Couldn't send request");
            // ...Response
                // Write response to buffer
        let response_buffer: &mut [u8; 1024 * 16] = &mut [0; 16384];
        tcp.read(response_buffer).map_err(|_| ())?;
            // Convert response to string
        let response_string = String::from_utf8(response_buffer.to_vec()).expect("Couldn't convert response to UTF-8 string").replace("\0", "");
        
            // ... Return result as slices i.e: OK;response_payload -> ["OK", "response_payload"]
        Ok(response_string.split(";")
            .into_iter()
            .map(|v| v.to_string())
            .collect::<Vec<_>>())
    }

    // Listen to show user tables list
    app.listen_global("show-tables", {
        let app_handler = app_handler.clone();
        let session_id_storage = session_id_storage.clone();
        let server_url_storage = server_url.clone();
        move |_| {
            // Handle request and get response
            match handle_dbs_request(format!("Show;what|x=x|database_tables 1-1 unit_name|x=x|none 1-1 session_id|x=x|{}", session_id_storage.lock().unwrap().to_owned()), server_url_storage.lock().unwrap().to_owned()) {
                Ok(resp_s) => {
                    // Rebound to frontend
                    if resp_s[0].to_lowercase() == "ok" {
                        // Send tables to frontend
                        app_handler.emit_all("show-tables-res", resp_s[1].as_str()).expect("Couldn't emit event"); // Send to frontend JSON object recived from database
                    }
                    else {
                        // Emitt that error
                        app_handler.emit_all("error", "Couldn't achive tables list from dbs").expect("Couldn't emit event");
                    }
                },
                Err(_) => app_handler.emit_all("error", "Couldn't read response from server").expect("Couldn't emit event")
            };
        }
    });

    // Download from "dbs" and send to "frontend" databases list
    app.listen_global("show-databases", {
        let app_handler = app_handler.clone();
        let session_id_storage = session_id_storage.clone();
        let server_url_storage = server_url.clone();
    
        move |_| {
            // Handle request and get response
            match handle_dbs_request(format!("Show;what|x=x|databases 1-1 unit_name|x=x|none 1-1 session_id|x=x|{}", session_id_storage.lock().unwrap().to_owned()), server_url_storage.lock().unwrap().to_owned()) {
                Ok(resp_s) => {
                    // Rebound to frontend
                    if resp_s[0].to_lowercase() == "ok" {
                        // Send databases to frontend
                        app_handler.emit_all("show-databases-res", resp_s[1].as_str()).expect("Couldn't emit event"); // Send to frontend JSON object recived from database
                    }
                    else {
                        // Emitt that error
                        app_handler.emit_all("error", "Couldn't achive list of databases from dbs").expect("Couldn't emit event");
                    }
                },
                Err(_) => app_handler.emit_all("error", "Couldn't read response from server").expect("Couldn't emit event")
            };
        }
    });

    // Connect user to specific database
    app.listen_global("connect-to-database", {
        let app_handler = app_handler.clone();
        let session_id_storage = session_id_storage.clone();
        let server_url_storage = server_url.clone();

        move |event| {
            let database_name = event.payload().unwrap().replace("\"", "");

            if database_name.len() > 0 {
                // Handle request and get response
                match handle_dbs_request(format!("DatabaseConnect;database_name|x=x|{} 1-1 session_id|x=x|{}", database_name, session_id_storage.lock().unwrap().to_owned()), server_url_storage.lock().unwrap().to_owned()) {
                    Ok(resp_s) => {
                        if resp_s[0].to_lowercase() == "ok" {
                            // Send that user is connected with database
                            app_handler.emit_all::<String>("connected-to-database", database_name).expect("Couldn't emit event"); // Send to frontend JSON object recived from database
                        }
                        else {
                            // Emitt error
                            let error_mes = {
                                if resp_s.len() > 1 {
                                    // Reason from server
                                    resp_s[1].as_str()
                                }
                                else {
                                    // Default reason
                                    "Couldn't change connection database"
                                }
                            };
                            app_handler.emit_all("error", error_mes).expect("Couldn't emit event");
                        }
                    },
                    Err(_) => app_handler.emit_all("error", "Couldn't read response from server").expect("Couldn't emit event")
                };
            }
        }
    });

    // Get table content
    app.listen_global("get-table-content", {
        let app_handler = app_handler.clone();
        let session_id_storage = session_id_storage.clone();
        let server_url_storage = server_url.clone();

        move |event| {
            let table_name = event.payload().unwrap().replace("\"", "");

            if table_name.len() > 0 {
                match handle_dbs_request(format!("Show;what|x=x|table_records 1-1 unit_name|x=x|{} 1-1 session_id|x=x|{}", table_name, session_id_storage.lock().unwrap().to_owned()), server_url_storage.lock().unwrap().to_owned()) { 
                    Ok(resp_s) => {
                        if resp_s[0].to_lowercase() == "ok" {
                            // Re-Send table content to user
                            app_handler.emit_all("table-content", resp_s[1].as_str()).expect("Couldn't emit event"); // Send to frontend JSON object recived from database
                        }
                        else {
                            // Emitt error
                            let error_mes = {
                                if resp_s.len() > 1 {
                                    // Reason from server
                                    resp_s[1].as_str()
                                }
                                else {
                                    // Default reason
                                    "Couldn't get table content"
                                }
                            };
                            app_handler.emit_all("error", error_mes).expect("Couldn't emit event");
                        }
                    },
                    Err(_) => app_handler.emit_all("error", "Couldn't read response from server").expect("Couldn't emit event")
                }
            }
        }
    });

    // Execute sql query
    app.listen_global("execute-sql-query", {
        let app_handler = app_handler.clone();
        let session_id_storage = session_id_storage.clone();
        let server_url_storage = server_url.clone();

        move |event| {
            let query_to_exe_str = event.payload().unwrap().replace("\"{", "{").replace("}\"", "}").replace("\\", "");

            if query_to_exe_str.len() > 0 {
                // Prepare SQL query to send it to dbs
                let query_to_exe_des = serde_json::from_str::<ExecuteSqlQuery>(&query_to_exe_str).unwrap();

                // SQL Query Example: UPDATE mycat2 SET name = 'hex', age = 255
                match handle_dbs_request(format!("Command;sql_query|x=x|{qu} 1-1 session_id|x=x|{sid}", sid = session_id_storage.lock().unwrap().to_owned(), qu = query_to_exe_des.query), server_url_storage.lock().unwrap().to_owned()) {
                    Ok(resp_s) => {
                        if resp_s[0].to_lowercase() == "ok" {
                            // Re-Send table content to user
                            let resm = if resp_s.len() > 1 {
                                resp_s[1].as_str()
                            }
                            else {
                                "query_performed"
                            };
                            app_handler.emit_all("execute-sql-query-successoutput", json!({ "result": resm, "execute_on_same_table": query_to_exe_des.execute_on_here })).expect("Couldn't emit event"); // Send to frontend JSON object recived from database with sql query results
                        }
                        else {
                            // Emitt error
                            let error_mes = {
                                if resp_s.len() > 1 {
                                    // Reason from server
                                    resp_s[1].as_str()
                                }
                                else {
                                    // Default reason
                                    "Couldn't execute query content"
                                }
                            };
                            app_handler.emit_all("error", error_mes).expect("Couldn't emit event");
                        }
                    },
                    Err(_) => app_handler.emit_all("error", "Couldn't read response from server").expect("Couldn't emit event")
                }
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
