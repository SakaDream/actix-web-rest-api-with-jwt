#[macro_use]
extern crate actix_web;
extern crate actix_rt;
#[macro_use]
extern crate log;
extern crate env_logger;

mod api;
mod config;

use actix_web::{
    HttpServer, App, middleware,
};
use std::{io, env};

fn main() -> io::Result<()> {
    env::set_var("RUST_LOG", "actix_web=debug");
    env_logger::init();

    let sys = actix_rt::System::new("address-book");

    HttpServer::new(|| {
        App::new()
            .wrap(middleware::Logger::default())
            .configure(config::config_services)
        })
    .bind("localhost:8080")?
    .start();

    sys.run()
}
