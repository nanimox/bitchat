use std::{
    fs::File,
    io::{Read, Write},
    vec,
};

use crate::user::person::Person;

pub struct Vault;

impl Vault {
    fn get_path(path: &String) -> String {
        format!("{}.txt", path.trim())
    }
    pub fn store(person: &Person, data: &String) {
        let path = Self::get_path(&person.address);
        match File::options().write(true).append(true).open(&path) {
            Ok(mut file) => Self::write_file(&mut file, &data),
            Err(_) => match File::create(&path) {
                Ok(mut file) => Self::write_file(&mut file, &data),
                Err(err) => eprintln!("Error creating file: {err}"),
            },
        }
    }
    pub fn write_file(file: &mut File, data: &String) {
        match file.write(format!("{data}\n").as_bytes()) {
            Ok(_) => return,
            Err(err) => eprintln!("Error writing data: {err}"),
        };
    }

    pub fn read(person: &Person) -> Vec<String> {
        let path: String = Self::get_path(&person.address);
        if let Ok(mut file) = File::open(path) {
            let mut buffer = String::new();
            file.read_to_string(&mut buffer);
            let splited = buffer.split("\n");
            let mut lines: Vec<String> = vec![];
            for msg in splited {
                println!(
                    "#####################################{msg}#####################################"
                );
                lines.append(&mut vec![String::from(msg)])
            }
            lines.pop();
            return lines;
        };
        vec![]
    }
}
