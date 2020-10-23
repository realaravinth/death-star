// Copyright (c) 2020 Aravinth T M <realaravinth@batsense.net>.
// See the COPYRIGHT file at the top-level directory of this
// distribution

//This program is free software; you can redistribute it and/or
//modify it under the terms of the GNU General Public License
//as published by the Free Software Foundation; either version 2
//of the License, or (at your option) any later version.

//This program is distributed in the hope that it will be useful,
//but WITHOUT ANY WARRANTY; without even the implied warranty of
//MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
//GNU General Public License for more details.

//You should have received a copy of the GNU General Public License
//along with this program; if not, write to the Free Software
//Foundation, Inc., 51 Franklin Street, Fifth Floor, Boston, MA  02110-1301, USA.
extern crate actix;
extern crate argon2;
extern crate config;
extern crate futures;
extern crate regex;
extern crate unicode_normalization;
extern crate uuid;
#[macro_use]
extern crate diesel;
extern crate env_logger;
extern crate num_cpus;
extern crate serde;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate lazy_static;
use crate::users::handler::index;
use actix_files::Files;
use actix_http::cookie::SameSite;
use actix_identity::{CookieIdentityPolicy, IdentityService};
use actix_session::CookieSession;
use actix_web::{
    middleware::{Compress, Logger},
    web, App, HttpServer,
};
use regex::Regex;
use std::env;

mod errors;
mod pow;
mod schema;
mod settings;
mod users;

use crate::settings::Settings;
use crate::users::filters::blacklist::tables::BLACKLIST;
use crate::users::filters::profainity::tables::PROFAINITY;
use crate::users::filters::user_case_mapped::tables::USERNAME_CASE_MAPPED;

use users::server;

lazy_static! {
    pub static ref SETTINGS: Settings = Settings::new().expect("couldn't load settings");
    pub static ref RE_BLACKLIST: Regex =
        Regex::new(BLACKLIST).expect("couldn't setup blacklist list filter");
    pub static ref RE_PROFAINITY: Regex =
        Regex::new(PROFAINITY).expect("coudln't setup profainity filter");
    pub static ref RE_USERNAME_CASE_MAPPED: Regex =
        Regex::new(USERNAME_CASE_MAPPED).expect("coudln't setup username case mapped filter");
    pub static ref ROOT: String =
        env::var("ROOT").expect("Please set ROOT to the port that you wish to listen to");
    pub static ref PORT: String = env::var("PORT")
        .expect("Please set PORT to the port that you wish to listen to")
        .parse()
        .expect("please enter valid time");
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    let port = env::var("PORT").expect("Please set PORT to the port that you wish to listen to");
    let secret = env::var("SECRET").expect("Please set SECRET");
    let domain =
        env::var("DOMAIN").expect("Please set DOMAIN to the port that you wish to listen to");
    let DATABASE_URL = env::var("DATABASE_URL")
        .expect("Please set DATABASE_URL to the port that you wish to listen to");

    env_logger::init();
    HttpServer::new(move || {
        App::new()
            .wrap(Compress::default())
            .wrap(
                CookieSession::signed(&secret.as_bytes())
                    .domain(&domain)
                    .name("shuttlecraft-session")
                    .path("/")
                    .secure(false),
            )
            .wrap(
                CookieSession::signed(&secret.as_bytes())
                    .domain(&domain)
                    .name("on")
                    .path("/")
                    .secure(false),
            )
            .wrap(IdentityService::new(
                CookieIdentityPolicy::new(secret.as_bytes())
                    .name("Authorization")
                    .max_age(20)
                    .domain(&domain)
                    .same_site(SameSite::Lax)
                    .secure(true),
            ))
            .route("/", web::get().to(index))
            .configure(server::config)
            .wrap(Logger::default())
    })
    .bind(format!("0.0.0.0:{}", &port))
    .expect(&format!(
        "Couldn't bind to IP address: 0.0.0.0 and port: {}, are they avaiable?",
        &port
    ))
    .run()
    .await
}
