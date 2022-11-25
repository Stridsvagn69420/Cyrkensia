use serde::{Deserialize, Serialize};
use rand::distributions::Alphanumeric;
use rand::{Rng, thread_rng};
use blake3::{Hasher, OUT_LEN};
use std::fmt::Display;

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
	pub fn new(name: String, passwd: String) -> Account {
		let salt = random_salt();
		Account {
			username: name,
			password: hash_passwd_salt(passwd, &salt),
			salt
		}
	}

	/// Verify Password
	/// 
	/// Verifies the Account against a given plaintext password.
	pub fn verify(&self, passwd: String) -> bool {
		let passhash = hash_passwd_salt(passwd, &self.salt);
		passhash == self.password
	}
}

impl Display for Account {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}:{}", self.username, self.password)
    }
}

/// Hash input
/// 
/// Hashes an input. The input has to append the salt itself.
/// Returns a formatted String representing the hash value.
pub fn hash(input: &[u8]) -> String {
	let mut hasher = Hasher::new();
	hasher.update(input);
	hasher.finalize().to_string()
}

/// Hash Password with Salt
/// 
/// Hashes the given password with given salt.
/// Wrapper for [hash].
pub fn hash_passwd_salt(passwd: String, salt: &String) -> String {
	// Get input and salt as bytes
	let mut userpass: Vec<u8> = passwd.into();
	userpass.extend(salt.as_bytes());
	hash(&userpass)
}

/// Random Salt
/// 
/// Generates a random salt for the given bit length.
/// Note that `bits` should be a multiple of 8.
pub fn random_salt() -> String {
	thread_rng()
	.sample_iter(&Alphanumeric)
	.take(OUT_LEN)
	.map(char::from)
	.collect()
}