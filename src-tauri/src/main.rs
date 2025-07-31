#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
mod crypto;
mod db;
mod user;

use std::{thread, time::Duration};

use crate::crypto::rsa::RSA;
use user::person::Person;

fn run() {
    tauri::Builder::default()
        .setup(|app| {
            if cfg!(debug_assertions) {
                app.handle().plugin(
                    tauri_plugin_log::Builder::default()
                        .level(log::LevelFilter::Info)
                        .build(),
                )?;
            }
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

fn main() {
    let user_input_thread = thread::spawn(|| -> String {
        println!("Enter your message: ");
        let mut user_input: String = String::new();
        match std::io::stdin().read_line(&mut user_input) {
            Ok(_) => return user_input.trim().to_string(),
            Err(_) => return "".to_string(),
        };
    });

    // This takes forever to load
    let mut person1 = Person::new(String::from("person1"));
    let person2 = Person::new(String::from("person2 "));

    while !user_input_thread.is_finished() {
        println!("loading...");
        thread::sleep(Duration::from_millis(500));
    }
    let user_input = user_input_thread.join().unwrap();
    RSA::encrypt(&mut person1, user_input, &person2);
    println!("{:?}", RSA::decrypt(&person2));
}
