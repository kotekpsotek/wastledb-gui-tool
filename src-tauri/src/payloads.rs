use serde::{ self, Serialize, Deserialize };


#[derive(Serialize, Deserialize, Debug)]
/// Recived from frontend when user send data to establish connection with database
pub struct EstablishConnection {
    #[serde(rename = "serverUrl", default)]
    pub server_url : String,
    #[serde(rename = "userName", default)]
    pub user_name: String,
    #[serde(rename = "userPassword", default)]
    pub user_password: String,
    #[serde(rename = "databaseName", default)]
    pub db_name: Option<String>,
    #[serde(rename = "createEncryptedConnection")]
    pub create_encrypted_connection: bool,
    #[serde(rename = "rsapublicKey", default)]
    pub rsapublic_key: Option<String>
}

#[derive(Debug, Deserialize)]
pub struct ExecuteSqlQuery {
    pub query: String,
    pub execute_on_here: bool
}
