use std::fmt::Display;
use std::path::Path;
use std::{io, fs};
use serde::{Deserialize, Serialize};
use serde_json::from_str;
use argon2::password_hash::rand_core::OsRng;
use argon2::password_hash::{SaltString, Result};
use argon2::{Argon2, PasswordHasher, PasswordVerifier, PasswordHash};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Account {
	/// Username
	/// 
	/// The account username.
	pub username: String,

	/// Password
	/// 
	/// The BLAKE3-hashed password and salt.
	pub password: String,

	/// Hash Salt
	/// 
	/// The salt used for hashing.
	pub salt: String
}

impl Account {
	/// New Account
	/// 
	/// Creates a new account with a random salt and given password.
	pub fn new(name: String, passwd: String) -> Result<Account> {
		let salt = random_salt();
		let hash = hash_passwd_salt(passwd, &salt)?.to_string();
		Ok(Account {
			username: name,
			password: hash,
			salt
		})
	}

	/// Verify Password
	/// 
	/// Verifies the Account against a given plaintext password.
	pub fn verify(&self, passwd: String) -> Result<()> {
		let passhash = hash_passwd_salt(passwd, &self.salt)?;
		get_argon2()
		.verify_password(self.password.as_bytes(), &passhash)
	}

	/// Load Account file
	/// 
	/// Loads an Account file
	pub fn load(file: impl AsRef<Path>) -> io::Result<Vec<Account>> {
		let data = fs::read_to_string(file)?;
		Ok(from_str(data.as_str())?)
	}
}

impl Display for Account {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}:{}", self.username, self.password)
    }
}

/// Hash Password with Salt
/// 
/// Hashes the password with given salt to a [String].
/// Wrapper for [Argon2]'s `hash_password()`.
pub fn hash_passwd_salt(passwd: String, salt: &String) -> Result<PasswordHash> {
	get_argon2()
	.hash_password(passwd.as_bytes(), salt)
}

/// Random Salt
/// 
/// Generates a random salt for Argon2 with the [OsRng].
pub fn random_salt() -> String {
	SaltString::generate(&mut OsRng)
	.to_string()
}

/// Get Argon2 context
/// 
/// Creates an Argon2 context that is used for every function here.
pub fn get_argon2() -> Argon2<'static> {
	Argon2::default()
}