#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
mod crypto;
mod db;
mod user;

use user::person::Person;

use crate::crypto::rsa::RSA;

fn main() {
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
        .invoke_handler(tauri::generate_handler![cycle])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
    #[tauri::command]
    fn cycle(msg: String) -> String{
        let mut person1 = Person::new(String::from("person1"));
        let person2 = Person::new(String::from("person2 "));
        RSA::encrypt(&mut person1, msg, &person2);
        return RSA::decrypt(&person2)[0].clone();
    }
}
