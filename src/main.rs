use clap::{arg, command, value_parser};
use std::io::prelude::*;

fn main() {
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

    if let Some(file) = matches.get_one::<std::path::PathBuf>("file") {
        println!("Value for file: {}", file.display());
        // create a file
        let mut file = std::fs::File::create(file).expect("Something went wrong in creating file");
        // write something to file
        // let File = matches
        //     .get_one::<std::path::PathBuf>("file")
        //     .as_path()
        //     .display()
        //     .to_string()
        //     .unwrap();
        let author = matches.get_one::<String>("author").unwrap();
        let mut author_string = String::new();
        author_string.push_str(&format!("Author : {}", author));
        let mut date_string = String::new();
        date_string.push_str(&format!("Date : {}", chrono::Local::now().to_rfc2822()));
        // let mut file_string = String::new();
        // file_string.push_str(&format!("File : {}", File));
        let mut data = String::new();
        data.push_str(&format!("/*{:*^50}*\\\n", ""));
        data.push_str(&format!("/*{:^50}*\\\n", author_string));
        data.push_str(&format!("/*{:^50}*\\\n", date_string));
        data.push_str(&format!("/*{:*^50}*\\\n", ""));
        file.write(data.as_bytes())
            .expect("Something went wrong in writing to file");
    }
    // Continued program logic goes here...
}
