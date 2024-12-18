# Password Manager in Rust

A **secure and efficient password manager** built with Rust, leveraging **Google Firestore** as the database. This program allows users to safely store, manage, evaluate, and generate passwords, offering a simple and user-friendly interface.

---

## Features

1. **User Authentication with Google OAuth2**:
   - Secure login using Google accounts.
   - Ensures that user data remains private and tied to their Google account.

2. **Password Management**:
   - **List Saved Passwords**: View all stored accounts and credentials.
   - **Add New Passwords**: Securely store new account details.
   - **Retrieve Passwords**: Easily retrieve stored passwords for accounts.
   - **Update Passwords**: Modify existing account credentials.
   - **Remove Passwords**: Delete credentials that are no longer needed.

3. **Password Strength Checking**:
   - Passwords are evaluated for their **entropy** to determine their strength.
   - Feedback is provided based on entropy levels:
     - ***Very Weak***: High risk of being guessed or cracked.
     - ***Weak*** Still vulnerable to attacks, improvement needed.
     - ***Strong*** Provides good security but could be improved.
     - ***Very Strong*** Highly secure password, ideal for use.

4. **Password Generation**:
   - Generate strong, random passwords to enhance security.

5. **Firestore Integration**:
   - Data is securely stored in Google Firestore, ensuring reliability and accessibility.

6. **User-Friendly Interface**:
   - Simple command-line interface with intuitive options.

---

## Getting Started

### Prerequisites
- Install [Rust](https://www.rust-lang.org/tools/install).
- Set up a [Google Cloud Firestore](https://cloud.google.com/firestore) project.
- Create a service account and download the JSON credentials file.
- Add the following environment variables:
  - `PROJECT_ID`: Your Firestore project ID.
  - `JSON_PATH`: Path to your JSON credentials file.
  - `GOOGLE_APPLICATION_CREDENTIALS`: Set automatically in the code.
  - `ID_CLIENT`: The Client ID from your Google Cloud OAuth 2.0 credentials.
  - `CLIENT_SECRET`: The Client Secret from your Google Cloud OAuth 2.0 credentials.

#### Setting Up Google OAuth 2.0 Credentials
1. Go to the [Google Cloud Console](https://console.cloud.google.com/).
2. Navigate to **APIs & Services** > **Library**.
2. Look for **Identity and Access Management (IAM) API** and enable it.
2. Then navigate to **APIs & Services** > **Credentials**.
3. Create a new **ID client OAuth 2.0**:
   - In this case, I choose "Desktop Application" as the application type.
   - Set the authorized redirect URI to: `http://localhost:8080/callback`.
4. Download the credentials file and extract the `Client ID` and `Client Secret`.
5. Set `ID_CLIENT` and `CLIENT_SECRET` in your `.env` file or as environment variables.

### Example `.env` File
Here’s an example of how your `.env` file should look:
```plaintext
PROJECT_ID=your_project_id
JSON_PATH=/path/to/your/credentials.json
ID_CLIENT=your_google_oauth_client_id
CLIENT_SECRET=your_google_oauth_client_secret
```

### Installation
1. Clone this repository:
   ```bash
   git clone https://github.com/JackZuno/PasswordManager.git
   cd password-manager
   ```
2. Build and run the code:
    ```bash
   cargo build
   cargo run
   ```

## Step-by-Step Walkthrough

This section provides a detailed walkthrough of the program’s functionality, complete with examples and visual aids. Follow along to see what you can do with this command-line password manager!

---

### 1. **Start the Program**
When you start the program, you’ll see the **main authentication menu** with options to log in, get help, or exit the program.

<center>
    <img src="/password-manager/images/menu_1.png" alt="Placeholder for main menu image" width="50%">
</center>

<div style="text-align: center;">
    <img src="/password-manager/images/menu_1.png" alt="Placeholder for main menu image" width="50%">
</div>

---

### 2. **Log in with Google OAuth2**
To log in, select **Option [1]**: Log in with Google. You’ll be given a URL to open in your browser for authentication. After authorizing the app, you’ll return to the program.

<div style="text-align: center;">
    <img src="/password-manager/images/user_auth.png" alt="Placeholder for Google login step image" width="100%">
</div>

---

### 3. **Access the Main Menu**
Once logged in, you’ll enter the **main menu**, where you can manage your saved passwords, generate new ones, or log out.

<div style="text-align: center;">
    <img src="/password-manager/images/main_menu.png" alt="Placeholder for main menu after login image" width="80%">
</div>

---

### 4. **List Saved Passwords**
To view your stored credentials, select **Option [1]** from the main menu. You’ll see a table listing all saved accounts and associated details.

<div style="text-align: center;">
    <img src="/password-manager/images/list_passwords.png" alt="Placeholder for listing passwords image" width="80%">
</div>

---

### 5. **Add a New Password**
Select **Option [2]** to add a new account and password. Follow the prompts to input the account name, username, and password. 
- **Master Password Requirement**: You will be prompted to provide your **master password**. This is a strong, secure password that you set and only you know. It is used to encrypt the newly added password to keep it safe.

- **Password Strength Feedback**: After entering the new password, the program will calculate its entropy and provide feedback on whether it is **very weak**, **weak**, **strong**, or **very strong**.

<div style="text-align: center;">
    <img src="/password-manager/images/add_new_account.png" alt="Placeholder for adding a new password image" width="70%">
</div>

---

### 6. **Retrieve a Password**
To retrieve a password, choose **Option [3]**. Enter the name of the account, and the program will display the stored credentials.
- **Master Password Requirement**: Before the password is decrypted and displayed, you’ll need to provide your **master password** to ensure security.

<div style="text-align: center;">
    <img src="/password-manager/images/retrieve_password.png" alt="Placeholder for retrieving a password image" width="80%">
</div>

---

### 7. **Update a Password**
Choose **Option [4]** to update an existing password. Select the account you want to update and provide the new password. \
To add an additional security layer, it is possible to verify if the user knows the password before delete it by asking him to insert the *master password* and the *password*. 
- **Master Password Requirement**: You’ll need to enter the **master password** to decrypt the existing password and securely replace it with the new one.
- **Password Strength Feedback**: The program will re-evaluate the strength of the new password to ensure it meets security standards.

<div style="text-align: center;">
    <img src="/password-manager/images/update_password.png" alt="Placeholder for updating a password image" width="70%">
</div>

---

### 8. **Remove a Password**
To delete a password, select **Option [5]**. You’ll be prompted to confirm the removal of the account and its associated credentials. To add an additional security layer, it is possible to verify if the user knows the password before delete it by asking him to insert the *master password* and the *password*.

<div style="text-align: center;">
    <img src="/password-manager/images/remove_password.png" alt="Placeholder for removing a password image" width="70%">
</div>

---

### 9. **Generate a Secure Password**
Select **Option [6]** to generate a random, secure password. The generated password will automatically be displayed, ensuring high entropy and security.

<div style="text-align: center;">
    <img src="/password-manager/images/password_generator.png" alt="Placeholder for generating a password image" width="60%">
</div>

---

### 10. **Log Out**
When you’re done managing your passwords, select **Option [7]** to log out and return to the authentication menu.

<div style="text-align: center;">
    <img src="/password-manager/images/logout.png" alt="Placeholder for logging out image" width="40%">
</div>

---

### 11. **Help**
When you’re back in the starting menu, select **Option [2]** to get more info about the program.

<div style="text-align: center;">
    <img src="/password-manager/images/help.png" alt="Placeholder for help image" width="80%">
</div>

---

### 12. **Exit**
When you’re done managing your passwords and back in the starting menu, select **Option [3]** to close the password manager.

<div style="text-align: center;">
    <img src="/password-manager/images/exit.png" alt="Placeholder for exit image" width="40%">
</div>

---

### Example Workflow
Here’s an example of how you might use the program in practice:
1. **Log in** with Google.
2. **Add a new password** for a new account.
3. **Generate a secure password** for another account.
4. **Retrieve a stored password** when you need it.
5. **Log out** to keep your data secure.

Each step is straightforward, with a user-friendly interface guiding you throughout.


