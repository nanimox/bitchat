use std::str::from_utf8;

pub struct Utils;

impl Utils {
    pub fn stringfy(vec: Vec<u8>) -> String {
        match from_utf8(&vec) {
            Ok(str) => String::from(str),
            Err(_) => String::from(""),
        }
    }

    pub fn num_vec_to_x_string(data: &Vec<u8>) -> String {
        let mut done: String = String::new();
        for item in data {
            done.push_str(&format!("{item}x"));
        }
        done
    }

    pub fn x_string_to_num_vec(data: &String) -> Vec<u8> {
        let sla = data.split('x');
        let mut full_data: Vec<u8> = vec![];
        for item in sla {
            full_data.push(item.parse().unwrap_or(0));
        }
        full_data.pop();
        full_data
    }
}
