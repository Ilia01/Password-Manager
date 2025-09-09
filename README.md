# Password Manager

A simple CLI password manager using **Argon2 key derivation** and **AES-256-GCM encryption** to securely store and retrieve passwords.

## Features

* Store passwords encrypted with a master password
* Generate strong random passwords
* Copy passwords from clipboard
* Warn about weak passwords
* Secure key derivation using Argon2

## Installation

```bash
git clone https://github.com/Ilia01/Password-Manager.git
cd password_manager

cargo build --release
```

## Usage

Run the compiled binary:

```bash
./target/debug/password_manager <command> [flags]
```

### Commands

* **List all passwords**:

```bash
./target/debug/password_manager list
```

* **Add a new password**:

```bash
./target/debug/password_manager add -u <username> -s <service> [flags]
```

### Flags for `add` command

| Flag              | Description                       |
| ----------------- | --------------------------------- |
| `-g, --generate`  | Generate a strong random password |
| `-c, --clipboard` | Use password from clipboard       |
| `-w, --write`     | Manually type the password        |
| `-u, --username`  | Username for the service          |
| `-s, --service`   | Service name                      |

### Examples

Generate a new password for GitHub:

```bash
./target/debug/password_manager add -u myusername -s github -g
```

Use a password from the clipboard for Gmail:

```bash
./target/debug/password_manager add -u myemail@gmail.com -s gmail -c
```

## Security Notes

* Master password is **never stored**.
* A **persistent salt** is stored in `salt.bin` to derive the encryption key.
* Each password has a **unique nonce** for AES-GCM encryption.
* Weak passwords trigger a warning; you can choose to continue or abort.

## Dependencies

* `aes-gcm` for AES-256-GCM encryption
* `rand` for cryptographically secure random number generation
* `argon2` for key derivation
* `clap` for CLI argument parsing
* `copypasta` for clipboard support
* `crossterm` for password warnings and input
