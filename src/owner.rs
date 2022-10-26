use serde::{Serialize, Deserialize};

/// Owner
/// 
/// The datatype for Cyrkensia owners.
#[derive(Serialize, Deserialize, Debug)]
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

// TODO: Add traits and other useful functions here