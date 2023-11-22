use std::fs;
use log::*;

fn main() {
    env_logger::init();
    info!("Game started");
    sql_connect("dirman_rpg.sql").expect("error");
    file_creator("test", true);
}

fn sql_connect(db_name: &str){

}

fn file_creator(file_name: &str, file_type: bool) {
    info!("File-Creator Called");
    match file_type {
        true => {
            info!("Creating directory named \"{file_name}\"");
        }
        false => {
            info!("Creating document named \"{file_name}\"");
        }
    }
}