use crate::models::user::User;
use crate::storage::{load_users, save_users};
use argon2::password_hash::{SaltString, rand_core::OsRng};
use argon2::{Argon2, PasswordHash, PasswordHasher, PasswordVerifier};

pub fn signup(username: &str, password: &str) {
    let mut users = load_users();

    if users.iter().any(|u| u.username == username) {
        println!("User already exists!");
    }

    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();

    let password_hash = argon2
        .hash_password(password.as_bytes(), &salt)
        .expect("Failed to hash password")
        .to_string();

    let new_user = User::new(username.to_string(), password_hash);
    users.push(new_user);

    if let Err(e) = save_users(&users) {
        eprintln!("Failed to save user: {}", e);
    } else {
        println!("User registered successfully!")
    }
}

pub fn login(username: &str, password: &str) {
    let users = load_users();

    match users.iter().find(|u| u.username == username) {
        Some(user) => {
            let parsed_hash = PasswordHash::new(&user.password_hash);
            match parsed_hash {
                Ok(hash) => {
                    match Argon2::default().verify_password(password.as_bytes(), &hash) {
                        Ok(_) => println!("Welcome, {}", username),
                        Err(_) => println!("Invalid password"),
                    }
                },
                Err(_) => println!("Invalid hash")
            }
        }

        None => println!("User '{}' not found", username),
    }
}
