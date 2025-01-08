use std::io;

use actix_cors::Cors;
use actix_files as fs;
use actix_session::storage::CookieSessionStore;
use actix_session::SessionMiddleware;
use actix_web::{cookie::Key, middleware, App, HttpResponse, HttpServer, Result};
use actix_web::{get, http, post};
use actix_web_validator::Json;
use lettre::message::{header::ContentType, Mailbox};
use lettre::transport::smtp::authentication::Credentials;
use lettre::{Message, SmtpTransport, Transport};
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Serialize, Deserialize, Validate)]
struct ContactUsFormData {
    #[validate(email)]
    email: String,
    subject: String,
    message: String,
}

#[get("/favicon")]
async fn favicon() -> Result<fs::NamedFile> {
    Ok(fs::NamedFile::open("./frontend/public/favicon.ico")?)
}

#[post("/api/contact/send-message")]
async fn send_message(data: Json<ContactUsFormData>) -> HttpResponse {
    let smtp_address = "smtp.gmail.com";
    let username = include_str!("../gmail-smtp-username.txt").trim();
    let password = include_str!("../gmail-smtp-password.txt").trim();

    match data.email.parse::<Mailbox>() {
        Ok(email_from) => {
            let email = Message::builder()
                .from(email_from.clone())
                .reply_to(email_from)
                .to("contact@muon.co".parse().unwrap())
                .subject(data.subject.clone())
                .header(ContentType::TEXT_PLAIN)
                .body(data.message.clone())
                .unwrap();

            let credentials = Credentials::new(username.to_owned(), password.to_owned());

            let mailer = SmtpTransport::relay(smtp_address)
                .unwrap()
                .credentials(credentials)
                .build();

            log::info!("{:?}", email);
            match mailer.send(&email) {
                Ok(_) => log::info!("Email sent successfully!"),
                Err(e) => log::error!("Failed to send email: {}", e),
            }
        }
        Err(e) => {
            log::error!("invalid email address: {}", e);
        }
    }

    HttpResponse::Ok().json(data.0)
}

#[actix_rt::main]
async fn main() -> io::Result<()> {
    // env::set_var("RUST_LOG", "actix_web=debug,actix_server=info");
    env_logger::init();

    HttpServer::new(|| {
        let secret_key = Key::generate();
        let cors = Cors::default()
            .allowed_origin("http://127.0.0.1:1111")
            .allowed_origin_fn(|origin, _req_head| {
                origin.as_bytes().starts_with(b"https://muon.co")
                    || origin.as_bytes().starts_with(b"http://localhost")
                    || origin.as_bytes().starts_with(b"http://127.0.0.1")
            })
            .allowed_methods(vec!["GET", "POST"])
            .allowed_headers(vec![http::header::AUTHORIZATION, http::header::ACCEPT])
            .allowed_header(http::header::CONTENT_TYPE)
            .max_age(3600);
        App::new()
            .wrap(cors)
            .wrap(SessionMiddleware::new(
                CookieSessionStore::default(),
                secret_key.clone(),
            ))
            .wrap(middleware::Logger::default())
            .service(favicon)
            .service(send_message)
            .service(fs::Files::new("/", "./frontend/public").index_file("index.html"))
    })
    .bind("127.0.0.1:1111")?
    .run()
    .await
}
