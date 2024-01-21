use std::fmt;
use argon2::{Argon2, PasswordHasher};
use argon2::password_hash::rand_core::OsRng;
use argon2::password_hash::{Result, SaltString};
use argon2::password_hash::PasswordHashString;
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
	/// It already includes the hash salt, so no additonal field is needed.
	pub password: String
}

impl User {
	/// Construct User
	/// 
	/// Just a constructor to create a [User].
	pub fn new(username: String, password: String) -> Self {
		Self {
			username,
			password
		}
	}

	/// Create User
	/// 
	/// Creates a new User Account from a username and plaintext password that gets hashed.
	pub fn create(user: &str, passwd: &[u8]) -> Result<User> {
		let salt = SaltString::generate(&mut OsRng);
		let hash = Argon2::default().hash_password(passwd, &salt)?;
		Ok(User {
			username: user.to_string(),
			password: hash.to_string()
		})
	}
}

impl From<(String, PasswordHashString)> for User {
    fn from(value: (String, PasswordHashString)) -> Self {
        Self::new(value.0, value.1.to_string())
    }
}

impl fmt::Display for User {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "{}:{}", self.username, self.password)
	}
}