#![allow(unused_must_use)]

#[macro_use]
extern crate actix_web;
#[macro_use]
extern crate log;
#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_migrations;
#[macro_use]
extern crate serde_derive;
extern crate actix_rt;
extern crate env_logger;
extern crate serde;
extern crate serde_json;
extern crate dotenv;
extern crate futures;
extern crate failure;
extern crate derive_more;

mod api;
mod config;
mod constants;
mod error;
mod models;
mod schema;
mod services;

use actix_web::{HttpServer, App, middleware};
use std::{io, env};

fn main() -> io::Result<()> {
    dotenv::dotenv().expect("Failed to read .env file");
    env::set_var("RUST_LOG", "actix_web=debug");

    env_logger::init();

    let app_host = env::var("APP_HOST").expect("APP_HOST not found.");
    let app_port = env::var("APP_PORT").expect("APP_PORT not found.");
    let app_url = format!("{}:{}", &app_host, &app_port);
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL not found.");

    let sys = actix_rt::System::new("address-book");
    let pool = config::db::migrate_and_config_db(&db_url);

    HttpServer::new(move || {
        App::new()
            .data(pool.clone())
            .wrap(middleware::Logger::default())
            .configure(config::app::config_services)
        })
    .bind(&app_url)?
    .start();

    info!("Server is started at {}", &app_port);

    sys.run()
}
