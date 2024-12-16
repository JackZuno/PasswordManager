// Include the rand crate in your Cargo.toml:
// [dependencies]
// rand = "0.8"

use rand::{thread_rng, Rng};
use rand::distributions::Alphanumeric;
use std::io;
use colored::*;

pub fn generate_random_password_wrapper() {
    let message = "Welcome to the Password Generator!".bold().bright_blue();
    println!("\n{}", message);

    let min_length = 8;
    let max_length = 128;

    println!("Enter the desired password length ({}-{}):", min_length, max_length);

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");

    let length: usize = match input.trim().parse() {
        Ok(num) if num >= min_length && num <= max_length => num,
        _ => {
            println!("Invalid input. Please enter a number between {} and {}.\n", min_length, max_length);
            return;
        }
    };

    let password = generate_random_password(length);
    println!("Your randomly generated password is: {}\n", password.yellow());

    let entropy = calculate_entropy(&password);
    println!("Password entropy: {:.4} bits", entropy);

    evaluate_password_strength(entropy);

    print!("\n");
}

fn generate_random_password(length: usize) -> String {
    let mut rng = thread_rng();
    let special_chars = r##"!#"$%&'()*+,-./:;<=>?@[\]^_`{|}~"##;


    (0..length)
        .map(|_| {
            if rng.gen_bool(0.8) {
                rng.sample(Alphanumeric) as char
            } else {
                special_chars.chars().nth(rng.gen_range(0..special_chars.len())).unwrap()
            }
        })
        .collect()
}


pub fn calculate_entropy(password: &str) -> f64 {
    let mut pool_size = 0;
    let mut has_lowercase = false;
    let mut has_uppercase = false;
    let mut has_digits = false;
    let mut has_special = false;

    for ch in password.chars() {
        if ch.is_lowercase() {
            has_lowercase = true;
        } else if ch.is_uppercase() {
            has_uppercase = true;
        } else if ch.is_digit(10) {
            has_digits = true;
        } else {
            has_special = true;
        }
    }

    if has_lowercase {
        pool_size += 26; // a-z
    }
    if has_uppercase {
        pool_size += 26; // A-Z
    }
    if has_digits {
        pool_size += 10; // 0-9
    }
    if has_special {
        pool_size += 32; // Special characters like !@#$...
    }

    let length = password.len();
    length as f64 * (pool_size as f64).log2()
}


pub fn evaluate_password_strength(entropy: f64) {
    let strength = if entropy < 36.0 {
        "Very Weak".red()
    } else if entropy >= 36.0 && entropy < 60.0 {
        "Weak".yellow()
    } else if entropy >= 60.0 && entropy < 120.0 {
        "Strong".green()
    } else {
        "Very Strong".bright_green()
    };

    println!("Password strength: {}", strength);
}
