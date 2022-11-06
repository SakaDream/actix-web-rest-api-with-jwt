#![allow(unused_must_use)]

use std::default::Default;
use std::{env, io};

use actix_cors::Cors;
use actix_web::dev::Service;
use actix_web::web;
use actix_web::{http, App, HttpServer};
use futures::FutureExt;

mod api;
mod config;
mod constants;
mod error;
mod middleware;
mod models;
mod schema;
mod services;
mod utils;

#[actix_rt::main]
async fn main() -> io::Result<()> {
    dotenv::dotenv().expect("Failed to read .env file");
    env::set_var("RUST_LOG", "actix_web=debug");
    env_logger::init();

    let app_host = env::var("APP_HOST").expect("APP_HOST not found.");
    let app_port = env::var("APP_PORT").expect("APP_PORT not found.");
    let app_url = format!("{}:{}", &app_host, &app_port);
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL not found.");

    let pool = config::db::init_db_pool(&db_url);
    config::db::run_migration(&mut pool.get().unwrap());

    HttpServer::new(move || {
        App::new()
            .wrap(
                Cors::default() // allowed_origin return access-control-allow-origin: * by default
                    .allowed_origin("http://127.0.0.1:3000")
                    .allowed_origin("http://localhost:3000")
                    .send_wildcard()
                    .allowed_methods(vec!["GET", "POST", "PUT", "DELETE"])
                    .allowed_headers(vec![http::header::AUTHORIZATION, http::header::ACCEPT])
                    .allowed_header(http::header::CONTENT_TYPE)
                    .max_age(3600),
            )
            .app_data(web::Data::new(pool.clone()))
            .wrap(actix_web::middleware::Logger::default())
            // .wrap(crate::middleware::auth_middleware::Authentication) // Comment this line if you want to integrate with yew-address-book-frontend
            .wrap_fn(|req, srv| srv.call(req).map(|res| res))
            .configure(config::app::config_services)
    })
    .bind(&app_url)?
    .run()
    .await
}

#[cfg(test)]
mod tests {
    use actix_cors::Cors;
    use actix_web::dev::Service;
    use actix_web::web;
    use actix_web::{http, App, HttpServer};
    use futures::FutureExt;
    use testcontainers::clients;
    use testcontainers::images::postgres::Postgres;

    use crate::config;

    #[actix_web::test]
    async fn test_startup_ok() {
        let docker = clients::Cli::default();
        let postgres = docker.run(Postgres::default());
        let pool = config::db::init_db_pool(
            format!(
                "postgres://postgres:postgres@127.0.0.1:{}/postgres",
                postgres.get_host_port_ipv4(5432)
            )
            .as_str(),
        );
        config::db::run_migration(&mut pool.get().unwrap());

        HttpServer::new(move || {
            App::new()
                .wrap(
                    Cors::default() // allowed_origin return access-control-allow-origin: * by default
                        // .allowed_origin("http://127.0.0.1:8080")
                        .send_wildcard()
                        .allowed_methods(vec!["GET", "POST", "PUT", "DELETE"])
                        .allowed_headers(vec![http::header::AUTHORIZATION, http::header::ACCEPT])
                        .allowed_header(http::header::CONTENT_TYPE)
                        .max_age(3600),
                )
                .app_data(web::Data::new(pool.clone()))
                .wrap(actix_web::middleware::Logger::default())
                .wrap(crate::middleware::auth_middleware::Authentication)
                .wrap_fn(|req, srv| srv.call(req).map(|res| res))
                .configure(config::app::config_services)
        })
        .bind("localhost:8000".to_string())
        .unwrap()
        .run();

        assert_eq!(true, true);
    }

    #[actix_web::test]
    async fn test_startup_without_auth_middleware_ok() {
        let docker = clients::Cli::default();
        let postgres = docker.run(Postgres::default());
        let pool = config::db::init_db_pool(
            format!(
                "postgres://postgres:postgres@127.0.0.1:{}/postgres",
                postgres.get_host_port_ipv4(5432)
            )
            .as_str(),
        );
        config::db::run_migration(&mut pool.get().unwrap());

        HttpServer::new(move || {
            App::new()
                .wrap(
                    Cors::default() // allowed_origin return access-control-allow-origin: * by default
                        // .allowed_origin("http://127.0.0.1:8080")
                        .send_wildcard()
                        .allowed_methods(vec!["GET", "POST", "PUT", "DELETE"])
                        .allowed_headers(vec![http::header::AUTHORIZATION, http::header::ACCEPT])
                        .allowed_header(http::header::CONTENT_TYPE)
                        .max_age(3600),
                )
                .app_data(web::Data::new(pool.clone()))
                .wrap(actix_web::middleware::Logger::default())
                .wrap_fn(|req, srv| srv.call(req).map(|res| res))
                .configure(config::app::config_services)
        })
        .bind("localhost:8001".to_string())
        .unwrap()
        .run();

        assert_eq!(true, true);
    }
}
