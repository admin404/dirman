#![allow(unused)]

use log::*;
use rusqlite::{Connection};
use home;
use std::{
    fs
};
use std::fs::create_dir_all;
use home::home_dir;

// struct Player {
//     name: String,
//     race: String,
//     class: String
// }

fn main() {
    env_logger::init();
    info!("Game started");
    sql_check();
    create_file("Dundeons & Dragons", true)
}

fn sql_check(){
    info!("sql_connect Called");

    let conn = Connection::open("db.sqlite");
    conn.unwrap().execute(
        "create table if not exists main.Players (playerName text not null)",
        (),
    ).expect("Error");
}

fn create_file(file_path: &str, file_type: bool) {
    info!("File-Creator Called");

    let mut home_dir = home_dir().to_owned();
    let binding = home_dir.unwrap();
    let home = binding.to_string_lossy() + "\\Desktop\\";
    let path = home + file_path;

    match file_type {
        true => {
            let path_override = path + "\\";
            info!("Creating directory: {path_override}");
            fs::create_dir_all(path_override.to_string());
        }
        false => {
            info!("Creating document: {path}");
        }
    }
}