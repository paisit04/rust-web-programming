#[macro_use]
extern crate rocket;

#[macro_use]
extern crate diesel;

use diesel::prelude::*;

use rocket::fairing::{Fairing, Info, Kind};
use rocket::http::Header;
use rocket::serde::json::Json;
use rocket::{Request, Response};

mod config;
mod database;
mod json_serialization;
mod jwt;
mod models;
mod schema;
mod to_do;
mod views;

use crate::database::DB;
use crate::json_serialization::to_do_items::ToDoItems;
use crate::models::item::item::Item;
use crate::models::item::new_item::NewItem;

use views::auth::{login::login, login::login_get, logout::logout};
use views::to_do::{create::create, delete::delete, edit::edit, get::get};
use views::users::create::create_user;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[get("/hello/<name>/<age>")]
fn hello(name: String, age: u8) -> String {
    format!("Hello, {} year old named {}!", age, name)
}

#[get("/bye/<name>/<age>")]
fn bye(name: String, age: u8) -> String {
    format!("Goodbye, {} year old named {}!", age, name)
}

#[post("/create/<title>")]
fn item_create(title: String, db: DB) -> Json<ToDoItems> {
    let items = schema::to_do::table
        .filter(schema::to_do::columns::title.eq(&title.as_str()))
        .order(schema::to_do::columns::id.asc())
        .load::<Item>(&db.connection)
        .unwrap();

    if items.len() == 0 {
        let new_post = NewItem::new(title, 1);
        let _ = diesel::insert_into(schema::to_do::table)
            .values(&new_post)
            .execute(&db.connection);
    }
    return Json(ToDoItems::get_state(1));
}

pub struct CORS;
#[rocket::async_trait]
impl Fairing for CORS {
    fn info(&self) -> Info {
        Info {
            name: "Add CORS headers to responses",
            kind: Kind::Response,
        }
    }
    async fn on_response<'r>(&self, _request: &'r Request<'_>, response: &mut Response<'r>) {
        response.set_header(Header::new("Access-Control-Allow-Origin", "*"));
        response.set_header(Header::new(
            "Access-Control-Allow-Methods",
            "POST, GET, PATCH, OPTIONS",
        ));
        response.set_header(Header::new("Access-Control-Allow-Headers", "*"));
        response.set_header(Header::new("Access-Control-Allow-Credentials", "true"));
    }
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index, hello, bye])
        .mount("/v1/item/", routes![create, delete, edit, get])
        .mount("/v1/auth/", routes![login, login_get, logout])
        .mount("/v1/user/", routes![create_user])
        .attach(CORS)
        .manage(CORS)
}
