use core::panic;
use std::{error, fmt::Error, fs::File, io::{self, Read}, fs::{self}};

fn main() {
    let greeting_file_result = File::open(
        "hello.txt"
    );

    let mut greeting_file =
    match greeting_file_result {
        Ok(file) => file,
        Err(error) => match error.kind() {
            std::io::ErrorKind::NotFound => 
            match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("failed to create file: {e:?}")
            },
            _ => panic!("Problem opening file: {error:?}")
        }
    };

    let file_contents_result = read_from_file(&mut greeting_file);
    let file_contents = file_contents_result.expect("Failed to read file");
    println!("{file_contents}");
    let file_txt = 
        match read_from_file_2("hello6.txt".to_string()) {
            Ok(text) => text,
            Err(err) => err.to_string()
        };
    println!("{file_txt}");
    let file_text = fs::read_to_string(&"hellog.txt".to_string())
        .unwrap_or_else(|e| e.to_string());
    println!("{file_text}");
}

fn read_from_file(file: &mut File) -> Result<String, io::Error> {
    let mut file_data = String::new();
    file.read_to_string(&mut file_data)?;
    Ok(file_data)
}

fn read_from_file_2(file_path: String) -> Result<String, io::Error>{
    let mut cnts = String::new();
    File::open(file_path)?.read_to_string(&mut cnts)?;
    Ok(cnts)
}

fn read_from_file_3(file_path: String) -> Result<String, io::Error> {
    fs::read_to_string(file_path)
}