// Module for hashing
mod hashing {
    use sha2::{Sha256, Digest};
    use hex::encode;

    // Function to hash data into sha256
    pub fn hash_data_to_sha256(data: String) -> String {
        // Create a hasher and feed the data into it
        let mut hasher = Sha256::new();
        hasher.update(data);

        // Convert to string
        let hashed_string = encode(hasher.finalize());

        hashed_string
    }
}

// Module for user data
mod user_data {
    use serde::{Deserialize, Serialize};

    // Data structure for registration/login
    #[derive(Deserialize, Serialize)]
    pub struct UserData {
        pub name: String,
        pub password: String
    }
}

// Module for JSON operations
mod json_operations {
    use crate::hashing::hash_data_to_sha256;
    use std::fs::File;
    use std::io::{Read, Write};
    use serde_json::{to_string, from_str};

    // Function to read data from JSON
    pub fn read_data_from_json(path: String) -> std::io::Result<String> {
        // Open file and create a new empty string
        let mut file = File::open(path)?;
        let mut data = String::new();

        // Read the file contents into the string
        file.read_to_string(&mut data)?;

        // Deserialize JSON string to regular string
        let readed_data_as_string: String = from_str(&data)?;

        Ok(readed_data_as_string)
    }

    // Function to write data to JSON
    pub fn write_data_to_json(data: String, path: String) -> std::io::Result<()> {
        // Hash the received string
        let hashed_string = hash_data_to_sha256(data);
        // Serialize the string into JSON
        let serialized_data = to_string(&hashed_string).unwrap();

        // Create a new file and write into it
        let mut file = File::create(path)?;
        file.write_all(serialized_data.as_bytes())?;

        Ok(())
    }
}

// Main module for server backend
mod web_backend {
    use crate::hashing::hash_data_to_sha256;
    use crate::json_operations::{read_data_from_json, write_data_to_json};
    use crate::user_data::UserData;
    use actix_web::{web, HttpResponse, Responder};
    use actix_files::NamedFile;

    // Function for the main page
    pub async fn index() -> actix_web::Result<NamedFile> {
        // Render index.html
        Ok(NamedFile::open("templates/index.html")?)
    }

    // Function for the profile page
    pub async fn profile() -> actix_web::Result<NamedFile> {
        // Render profile.html
        Ok(NamedFile::open("templates/profile.html")?)
    }

    // Function for login
    pub async fn login(login_data: web::Json<UserData>) -> impl Responder {

        // Read data from JSON and try to find the user
        let readed_data = match read_data_from_json(format!("{name}.json", name = login_data.name)) {
            Ok(result) => result,
            Err(_) => return HttpResponse::Unauthorized().body("User not found!"),
        };

        // Combine username and password into a single string
        let combined_string = login_data.name.clone() + login_data.password.as_str();

        // Hash the combined string
        let hashed_string = hash_data_to_sha256(combined_string);

        // If hashes match
        if hashed_string == readed_data {
            HttpResponse::Ok().body("Login successfully!")
        } else {
            HttpResponse::Unauthorized().body("Invalid login data!")
        }
    }

    // Function for sign up
    pub async fn sign_up(sign_data: web::Json<UserData>) -> impl Responder {
        // Combine username and password into a single string
        let combined_string = sign_data.name.clone() + sign_data.password.as_str();

        // Write the string into JSON
        match write_data_to_json(combined_string, format!("{name}.json", name = sign_data.name)) {
            Ok(()) => HttpResponse::Ok().body("User registered successfully!"),
            Err(e) => HttpResponse::InternalServerError().body(format!("Unexpected error!: {}", e))
        }
    }
}

use actix_web::{main, App, HttpServer, web};
use crate::web_backend::{index, profile, login, sign_up};

#[main]
async fn main() -> std::io::Result<()> {
    let port = 4444;

    // Start the server
    HttpServer::new(|| { App::new()
        .service(actix_files::Files::new("/static", "./static"))
        .route("/", web::get().to(index))
        .route("/profile", web::get().to(profile))
        .route("/login", web::post().to(login))
        .route("/signup", web::post().to(sign_up))
    })
    .bind(("127.0.0.1", port))?.workers(16).run().await
}
