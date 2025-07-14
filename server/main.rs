// Модуль для хэширования
mod hashing{
    use sha2::{Sha256, Digest};
    use hex::encode;

    // Функция для хэширования в sha 256
    pub fn hash_data_to_sha256(data: String) -> String{
        // Создаем хэшер и хэшируем туда данные
        let mut hasher = Sha256::new();
        hasher.update(data);

        // Переводим в строку
        let hashed_string = encode(hasher.finalize());

        hashed_string
    }
}

// Модуль с датой юзера
mod user_data{
    use serde::{Deserialize, Serialize};

    // Структура данных при регистрации/логине
    #[derive(Deserialize, Serialize)]
    pub struct UserData{
        pub name: String,
        pub password: String
    }
}

// Модуль для операций с json
mod json_operations {
    use crate::hashing::hash_data_to_sha256;
    use std::fs::File;
    use std::io::{Read, Write};
    use serde_json::{to_string, from_str};

    // Читает дату из json
    pub fn read_data_from_json(path: String) -> std::io::Result<String>{
        // Открывает файл и создает новую пустую строку
        let mut file = File::open(path)?;
        let mut data = String::new();

        // Записывает данные в data
        file.read_to_string(&mut data)?;

        // json-строку переводит в обычную
        let readed_data_as_string: String = from_str(&data)?;

        Ok(readed_data_as_string)
    }

    // Функция для записи в json
    pub fn write_data_to_json(data: String, path: String) -> std::io::Result<()>{
        // Хэшируем полученную строку
        let hashed_string = hash_data_to_sha256(data);
        // Сериализует эту строку в json
        let serialized_data = to_string(&hashed_string).unwrap();

        // Создает новый файл и пишет в него
        let mut file = File::create(path)?;
        file.write_all(serialized_data.as_bytes())?;

        Ok(())
    }
}

// Основной модуль для серверного бэкенда
mod web_backend{
    use crate::hashing::hash_data_to_sha256;
    use crate::json_operations::{read_data_from_json, write_data_to_json};
    use crate::user_data::UserData;
    use actix_web::{web, HttpResponse, Responder};
    use actix_files::NamedFile;

    // Функция для главной страницы
    pub async fn index() -> actix_web::Result<NamedFile> {
        // Рендерит index.html
        Ok(NamedFile::open("templates/index.html")?)
    }

    // Функция для профиля
    pub async fn profile() -> actix_web::Result<NamedFile> {
        // Рендерит profile.html
        Ok(NamedFile::open("templates/profile.html")?)
    }

    // Функция для логина
    pub async fn login(login_data: web::Json<UserData>) -> impl Responder{

        // Читаем дату из json'а и пытаемся найти юзера
        let readed_data = match read_data_from_json(format!("{name}.json", name = login_data.name)) {
            Ok(result  ) => result,
            Err(_) => return HttpResponse::Unauthorized().body("User not found!"),
        };

        // Совмещаем имя и пароль в одну строку(как в хэшировании)
        let combined_string = login_data.name.clone() + login_data.password.as_str();

        // Хэшируем
        let hashed_string = hash_data_to_sha256(combined_string);

        // Если хэши совпадают
        if hashed_string == readed_data{
            HttpResponse::Ok().body("Login successfully!")
        }
        else {
            HttpResponse::Unauthorized().body("Invalid login data!")
        }
    }

    // Функция для регистрации
    pub async fn sign_up(sign_data: web::Json<UserData>) -> impl Responder{
        // Совмещает имя и пароль
        let combined_string = sign_data.name.clone() + sign_data.password.as_str();

        // Пишет эту строку в json
        match write_data_to_json(combined_string, format!("{name}.json", name = sign_data.name)) {
            Ok(()) => HttpResponse::Ok().body("User registred successfully!"),
            Err(e) => HttpResponse::InternalServerError().body(format!("Unexpected error!: {}", e))
        }
    }
}

use actix_web::{main, App, HttpServer, web};
use crate::web_backend::{index, profile, login, sign_up};

#[main]
async fn main() -> std::io::Result<()>{
    let port = 4444;

    // Создаем сервер
    HttpServer::new(|| { App::new()
    .service(actix_files::Files::new("/static", "./static"))
    .route("/", web::get().to(index))

    .route("/profile", web::get().to(profile))

    .route("/login", web::post().to(login))
    .route("/signup", web::post().to(sign_up))})

    .bind(("127.0.0.1", port))?.workers(16).run().await
}