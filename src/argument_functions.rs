use std::fs::{read_to_string,File};
use std::io::prelude::*;
use super::utils::get_db_path;
use super::database_loader;
use super::cryptograpy;

pub fn import(args: Vec<String>){
    import_database(&args[2]);
}

pub fn help(){
    println!("Help");
}

pub fn add(args: Vec<String>){
    if args.len() == 5{
        if database_loader::add_element(&args[2],&args[3],&args[4]){
            println!("Success");
        }
        else{
            println!("Invalid values");
        }
    }
    else{
        println!("Invalid arguments, type cotp --add <secret> <issuer> <label>");
    }
}

pub fn remove(args: Vec<String>){
    if args.len() == 3{
        let id = args[2].parse::<usize>().unwrap();
        if database_loader::remove_element_from_db(id) {
            println!("ok");
        }
        else{
            println!("{} is a wrong index", id);
        }
    }
    else{
        println!("Invalid argument, type cotp --remove <index>");
    }
}

pub fn modify(args: Vec<String>){
    if args.len() == 6{
        let id = args[2].parse::<usize>().unwrap();
        let secret = &args[3];
        let issuer = &args[4];
        let label = &args[5];
        database_loader::modify_element(id, &secret, &issuer, &label).expect("An error occured");
    }
    else{
        println!("Invalid arguments, type cotp --modify <id> <secret> <issuer> <label>\n\nReplace the attribute value with \".\" to skip the attribute modification");
    }
}

pub fn export(args: Vec<String>){
    if args.len() == 2{
        let export_result = database_loader::export_database();
        match export_result{
            Ok(export_result) => {
                println!("Database was successfully exported at {}", export_result);
            },
            Err(e) =>{
                println!("An error as occured while exporting database: {}", e);
            }
        }
    }
    else{
        println!("Invalid argument, type cotp --export");
    }
}

fn import_database(filename: &String){
    let mut unencrypted_content = read_to_string(filename).unwrap();
    let encrypted_content = cryptograpy::encrypt_string(&mut unencrypted_content,&cryptograpy::prompt_for_passwords("Insert password for database encryption: "));
    let mut encrypted_file = File::create(&get_db_path()).expect("Cannot create encrypted database file");
    encrypted_file.write_all(encrypted_content.as_bytes()).expect("Cannot write to encrypted file");
    println!("Successfully imported database");
}