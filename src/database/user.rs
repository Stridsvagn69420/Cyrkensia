use std::fmt;
use argon2::{Argon2, PasswordHasher};
use argon2::password_hash::rand_core::OsRng;
use argon2::password_hash::{Result, SaltString};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct User {
	/// Username
	///
	/// The plaintext username.
	pub username: String,

	/// Hashed Password
	///
	/// The Argon2 PHC Hash as a [String].
	pub password: String
}

impl User {
	/// New User
	/// 
	/// Creates a new User Account from a username and plaintext password that gets hashed.
	pub fn new(user: &str, passwd: &[u8]) -> Result<Self> {
		let salt = SaltString::generate(&mut OsRng);
		let hash = Argon2::default().hash_password(passwd, &salt)?;
		Ok(Self {
			username: user.to_string(),
			password: hash.to_string()
		})
	}
}

impl fmt::Display for User {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "{}:{}", self.username, self.password)
	}
}