use serde::{Serialize, Deserialize};
use std::fmt::{Display, Result};
use std::cmp::PartialEq;
use std::convert::From;
use super::Artist;

/// Owner
/// 
/// The datatype for Cyrkensia owners.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Owner {
	/// Name
	/// 
	/// The username of the owner. This is optional.
	pub name: String,

	/// E-Mail
	/// 
	/// The email address of the owner. This is optional.
	pub email: Option<String>,

	/// Website
	/// 
	/// The website or profile of an owner. This is optional.
	pub website: Option<String>
}

impl Owner {
	/// New Owner
	/// 
	/// Creates a new owner. E-Mail and Website are optional, though if `email` isn't used, `website` should be, and vice versa.
	pub fn new(name: String, email: Option<String>, website: Option<String>) -> Owner {
		Owner {
			name,
			email,
			website
		}
	}
}

impl From<Artist> for Owner {
	fn from(x: Artist) -> Self {
		Owner {
			name: x.name,
			email: None,
			website: x.website
		}
	}
}

impl PartialEq for Owner {
	fn eq(&self, other: &Self) -> bool {
		self.name == other.name &&
		self.email == other.email &&
		self.website == other.website
	}
}

impl Display for Owner {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result {
		match (self.email.clone(), self.website.clone()) {
			(None, None) => {
				// Name only
				write!(f, "{}", self.name)
			},
			(None, Some(y)) => {
				// Name and Website
				write!(f, "{} ({})", self.name, y)
			},
			(Some(x), None) => {
				// Name and E-Mail
				write!(f, "{} <{}>", self.name, x)
			},
			(Some(x), Some(y)) => {
				// Name, E-Mail and Website
				write!(f, "{} <{}> ({})", self.name, x, y)
			}
		}
	}
}