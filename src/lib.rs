#![forbid(unsafe_code)]
use args::CotpArgs;
use clap::Parser;
use color_eyre::eyre::eyre;
use otp::otp_element::{OTPDatabase, CURRENT_DATABASE_VERSION};
use reading::{get_elements_from_input, get_elements_from_stdin, ReadResult};
pub use reading::get_elements_with_password;
use std::{error, vec};
use zeroize::Zeroize;

mod args;
mod argument_functions;
mod clipboard;
mod crypto;
mod exporters;
mod importers;
pub mod otp;
mod path;
mod reading;
mod utils;

/// Application result type.
pub type AppResult<T> = Result<T, Box<dyn error::Error>>;

pub fn init(read_password_from_stdin: bool) -> color_eyre::Result<ReadResult> {
    match utils::init_app() {
        Ok(first_run) => {
            if first_run {
                // Let's initialize the database file
                let mut pw = utils::verified_password("Choose a password: ", 8);
                let mut database = OTPDatabase {
                    version: CURRENT_DATABASE_VERSION,
                    elements: vec![],
                    ..Default::default()
                };
                let save_result = database.save_with_pw(&pw);
                pw.zeroize();
                save_result.map(|(key, salt)| (database, key, salt.to_vec()))
            } else if read_password_from_stdin {
                get_elements_from_stdin()
            } else {
                get_elements_from_input()
            }
        }
        Err(()) => Err(eyre!("An error occurred during database creation")),
    }
}

pub fn main() -> AppResult<()> {
    color_eyre::install()?;

    let cotp_args: CotpArgs = CotpArgs::parse();
    let (database, mut key, salt) = match init(cotp_args.password_from_stdin) {
        Ok(v) => v,
        Err(e) => {
            println!("{e}");
            std::process::exit(-1);
        }
    };

    let mut reowned_database = match args::args_parser(cotp_args, database) {
        Ok(d) => d,
        Err(e) => {
            eprintln!("An error occurred: {e}");
            key.zeroize();
            std::process::exit(-2)
        }
    };

    let error_code = if reowned_database.is_modified() {
        match reowned_database.save(&key, &salt) {
            Ok(_) => {
                println!("Modifications has been persisted");
                0
            }
            Err(_) => {
                eprintln!("An error occurred during database overwriting");
                -1
            }
        }
    } else {
        0
    };
    key.zeroize();
    std::process::exit(error_code)
}
