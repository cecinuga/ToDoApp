use actix_web::web;
use std::env;

mod to_do;
mod path;
mod auth;


pub fn views_factory(app: &mut web::ServiceConfig){
    auth::auth_factory(app);
    to_do::item_factory(app);
}