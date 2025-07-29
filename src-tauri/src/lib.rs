// Prevents additional console window on Windows in release, DO NOT REMOVE!!
//#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
//  app_lib::run();
mod person;
use std::str::from_utf8;

use person::Person;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
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
        .invoke_handler(tauri::generate_handler![app])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[tauri::command]
fn app(data: String) -> String {
    let mut person1 = Person::new(String::from("person1"));
    let person2 = Person::new(String::from("person2 "));

    println!("message: ");
    let enc_data = person1.encrypt(data, &person2.pub_key);
    let dec_data = person2.decrypt(&enc_data);

    //println!("dec data: {dec_data:?}");
    let result: String = match from_utf8(&dec_data) {
        Ok(val) => val.to_string(),
        Err(_) => String::new(),
    };
    println!("{result}");
    result
}
