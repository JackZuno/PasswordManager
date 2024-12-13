# Password Manager

### Rust Project
```bash
cargo new password-manager

cd password-manager


cargo build

cargo run main.rs
cargo run
```

### Firestore
In my case, with **wsl**, the path must be the one of wsl (*/mnt/c/*) and not the windows one.
```bash
export GOOGLE_APPLICATION_CREDENTIALS="/mnt/c/path/to/file/src/private/passwordmanager-c8b1f-firebase-adminsdk-u7rch-bc884141d0.json"
```

cbc: https://github.com/RustCrypto/block-modes