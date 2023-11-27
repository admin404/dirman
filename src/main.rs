use log::*;
use rusqlite::{Connection};

// struct Player {
//     name: String,
//     race: String,
//     class: String
// }

fn main() {
    env_logger::init();
    info!("Game started");
    sql_connect();
}

fn sql_connect(){
    info!("sql_connect Called");

    let conn = Connection::open("db.sqlite");
    conn.unwrap().execute(
        "create table if not exists main.palyer (
                name text not null primary key,
                race text not null,
                class text not null
            ) without rowid ;",
        (),
    ).expect("Error");
}

// fn file_creator(file_name: &str, file_type: bool) {
//     info!("File-Creator Called");
//     match file_type {
//         true => {
//             info!("Creating directory named \"{file_name}\"");
//         }
//         false => {
//             info!("Creating document named \"{file_name}\"");
//         }
//     }
// }

fn create_player(name: &str, race: &str, class: &str) {
    info!("Creating player \n named: {name}\n race: {race}\n class:{class}");
}