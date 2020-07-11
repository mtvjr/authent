#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;

// use std::path::{Path, PathBuf};
// use rocket::response::NamedFile;
use rocket_contrib::serve::StaticFiles;

mod login;

This is not valid rust;

fn main() {
    rocket::ignite()
        .mount("/", routes![login::route])
        .mount("/", StaticFiles::from("src/client"))
        .launch();
}
