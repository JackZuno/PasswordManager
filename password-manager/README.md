# How It Works

## Functionalities

### Add a New Password
The `add_password_function` stores user credentials securely:
1. **Inputs**: Website name, username, password, and a master password.
2. **Security Validation**:
   - Master password must be 12-128 characters.
   - Website and username must be 3-24 characters.
   - Password must be 8-128 characters.
3. **Password Strength Evaluation**: Entropy calculation helps users evaluate their password's strength.
4. **Encryption Process**:
   - A unique **nonce** and a **salt** are generated for each password.
   - The master password is combined with the salt to derive a **master key** using PBKDF2 with HMAC-SHA256.
   - The password is encrypted using AES-256-GCM, with the derived key and nonce.

### Retrieve a Password
The `retrieve_password_function` decrypts stored passwords:
1. **Inputs**: Website name, master password.
2. **Decryption Process**:
   - Re-derives the **decryption key** using the stored salt and the master password.
   - Decodes the stored encrypted password and nonce.
   - Decrypts the password using AES-256-GCM with the derived key and nonce.

### Generate Random Password
The `generate_random_password_wrapper` asks the user how long the password must be (8-128 characters). It combines alphanumeric characters and special symbols to ensure high entropy.\
The entropy is calculated with the function `calculate_entropy` (which is a measure of how unpredictable or secure the password is based on its length and the variety of characters it contains). The function checks if the password contains lowercase letters, uppercase letters, digits, or special characters by using a boolean for each type of character. \
After iterating through the characters in the password, the function calculates the pool_size, which represents the total number of possible characters that can be used to form the password:
- If there are lowercase letters, it adds 26 (for a-z).
- If there are uppercase letters, it adds 26 (for A-Z).
- If there are digits, it adds 10 (for 0-9).
- If there are special characters, it adds 32

The **entropy** is calculated using the formula: `Entropy = password length × log₂(pool size)`\
The **pool_size** is converted to a logarithmic scale using log2(), which calculates how many bits are needed to represent each character in the pool. The result is then multiplied by the length of the password to get the total entropy.\
Then the **password strenght** is categorized based on the given entropy:
- Very Weak if entropy is less than 36.0.
- Weak if entropy is between 36.0 and 60.0.
- Strong if entropy is between 60.0 and 120.0.
- Very Strong if entropy is 120.0 or higher.

### Remove Password
The `remove_password_function` ask the user to insert the website name and then it search it inside the database. If is it present it then procedes to delete it, if not it returns a *not found message*.
Before starting the deleting procedure, it ask the user for confermation.\
Other info:
1. **Inputs**: Website name, user. 


### Update Password
The `update_password_function`, givent the website name and the user info, extract the password from the database and, if is it exist, ask the user the master password and the new password.\
The new password is encrypted using a new salt and nonce, that are then store in the database updating the old ones.\
Other info:
1. **Inputs**: Website name, user, master password, new password.

### Listing Password
The `list_items_menu` function retrieves the passwords of the current user from the firestore database and then display them using **prettytable**.\
The info showed are the name of the website, the username used inside the website and the creation date and last modified date. These dates are automatically inserted when the user insert a new password and when it updates an existing one.\
Other info:
1. **Inputs**: user.

---

## Security Design

### Encryption Algorithms and Design Choices

#### 1. **AES-256-GCM**
- Used to encrypt and decrypt passwords.
- Provides authenticated encryption, ensuring both confidentiality and integrity.
- Resistant to tampering due to the authentication tag.

#### 2. **PBKDF2-HMAC-SHA256**
- Used to derive cryptographic keys from the master password.
- Iterated 600,000 times (as suggested by OWASP, the Open Worldwide Application Security Project) to slow down brute-force attacks.
- Ensures secure key derivation using a random 32-byte salt.

#### 3. **Nonce Generation**
- 12-byte random nonce (number used once) ensures each encryption operation is unique.
- Prevents replay attacks and guarantees the same plaintext won't produce identical ciphertexts.

#### 4. **Salt**
- 32-byte salt ensures master password-derived keys are unique for each password entry.
- Protects against precomputed attacks (e.g., rainbow tables).

### Why These Algorithms?
- **AES-256-GCM** is a modern encryption standard, widely regarded as secure.
- **PBKDF2** mitigates the risk of brute-force attacks by making key derivation computationally expensive.
- Using both a nonce and a salt ensures high security against cryptographic attacks.

---
