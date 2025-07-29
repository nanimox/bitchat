mod crypto;
mod db;
mod user;

use user::person::Person;

use crate::crypto::rsa::RSA;

fn main() {
    app_lib::run();
    let mut person1 = Person::new(String::from("person1"));
    let person2 = Person::new(String::from("person2 "));

    RSA::encrypt(&mut person1, "nanimo".to_string(), &person2);
    println!("{:?}", RSA::decrypt(&person2));
    RSA::encrypt(&mut person1, "is the".to_string(), &person2);
    println!("{:?}", RSA::decrypt(&person2));
    RSA::encrypt(&mut person1, "goat".to_string(), &person2);
    println!("{:?}", RSA::decrypt(&person2));
}
