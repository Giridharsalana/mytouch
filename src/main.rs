use clap::{arg, command, value_parser};
use std::io::prelude::*;

fn main() {
    // Clap Arg Parser
    let matches = command!()
        .arg(
            arg!([file])
                .required(true)
                .value_name("File Path")
                .value_parser(value_parser!(std::path::PathBuf))
                .index(1),
        )
        .arg(
            arg!(-a --author <author> "Pass the author name")
                .required(false)
                .value_parser(value_parser!(String)),
        )
        .get_matches();

    // Create a file with filename
    if let Some(file) = matches.get_one::<std::path::PathBuf>("file") {
        let mut file = std::fs::File::create(file).expect("Something went wrong in creating file");
        let mut author_string = String::new();

        // Set author name if its passed otherwise set it to none
        if let Some(author) = matches.get_one::<String>("author") {
            let _author = matches.get_one::<String>("author");
            author_string.push_str(&format!("Author : {:?}", author));
        } else {
            author_string.push_str(&format!("Author : None"));
        }

        // Set current date as file creation date
        let mut date_string = String::new();
        date_string.push_str(&format!("Date : {}", chrono::Local::now().to_rfc2822()));

        // Push all data to multiline string to be written in file
        let mut data = String::new();
        data.push_str(&format!("/*{:*^50}*\\\n", ""));
        data.push_str(&format!("/*{:^50}*\\\n", author_string));
        data.push_str(&format!("/*{:^50}*\\\n", date_string));
        data.push_str(&format!("/*{:*^50}*\\\n", ""));

        // Write everything to file
        file.write(data.as_bytes())
            .expect("Something went wrong in writing to file");
        // Continued program logic goes here...
    }
}
