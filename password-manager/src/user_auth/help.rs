use colored::*;

pub fn help_function() {
    println!("\n{}", "Welcome to Your Secure Password Manager!".bold().underline().bright_blue());
    println!();
    println!(
        "{}",
        "This program helps you securely manage your online accounts and passwords.".italic()
    );
    println!(
        "{}",
        "With the power of Google authentication and a user-friendly interface, you can:"
            .italic()
    );
    println!();
    println!(
        "\t{} {}",
        "[1]".green().bold(),
        "Log in securely using your Google account.".bright_white()
    );
    println!(
        "\t{} {}",
        "[2]".yellow().bold(),
        "Store, retrieve, manage, evaluate and generate your passwords effortlessly.".bright_white()
    );
    println!(
        "\t{} {}",
        "[3]".red().bold(),
        "Keep your sensitive information private and secure.".bright_white()
    );
    println!();
    println!(
        "{} {}",
        "Features:".bold().underline(),
        "Everything you need for password management:".dimmed()
    );
    println!();
    println!(
        "{} {}\n\t{} {}\n\t{} {}\n\t{} {}\n\t{} {}\n\t{} {}",
        "[✔]".bright_green(),
        "List all your saved accounts.".bold(),
        "[✔]".bright_green(),
        "Add new accounts and passwords.".bold(),
        "[✔]".bright_green(),
        "Retrieve stored passwords securely.".bold(),
        "[✔]".bright_green(),
        "Update existing account details.".bold(),
        "[✔]".bright_green(),
        "Remove accounts you no longer need.".bold(),
        "[✔]".bright_green(),
        "Generate and evalute a secure password.".bold()
    );
    println!();
    println!(
        "{}",
        "Get started now by choosing an option from the menu!\n".bright_blue(),
    );
}
