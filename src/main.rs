#![allow(warnings)]
#![feature(plugin)]
#![plugin(rocket_codegen)]

#[macro_use]
extern crate dump;


extern crate rocket;
use rocket::State;

use std::ops::Deref;
use std::path::PathBuf;



extern crate redis;
mod rd;







#[get("/<command>/<args..>")]
fn index(conn: State<redis::Client>, command: String, args: PathBuf) -> String {
    let qargs = args.iter()
        .map(|seg| String::from(seg.to_str().unwrap()))
        .collect::<Vec<String>>();
    rd::execute(conn.deref(), command, qargs).unwrap_or(String::from("Ok"))

}


fn main() {
    rocket::ignite()
        .manage(rd::create_connection("redis://127.0.0.1"))
        .mount("/", routes![index])
        .launch();
}
