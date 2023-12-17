
//! The rich error system for the project.
//! 
//! This module contains structs and enums
//! which define potential erroneous or
//! failure states for the commands that
//! interact with the databases.
//! 
//! Because Rust is statically typed, it helps
//! to wrap the potential states (that I use an
//! enum for) in another struct which has other
//! information and implementation aside from it.

use std::{
    error::Error,
    fmt::Display,
};

use serde::{
    Serialize, 
    Deserialize
};



#[derive(Debug, Serialize, Deserialize)]
pub struct UserReqError {
    kind: UserReqErrorKind,
    message: &'static str,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum UserReqErrorKind {
    InvalidDetails,
    AccountLocked,
    ConnectionError,
    AddUserError,
    StrikeAddError,
    StrikeResetError,
}

impl Display for UserReqError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "UserReqError [ {:?}, {} ]", self.kind, self.message)
    }
}

impl Error for UserReqError {}

impl UserReqError {
    /// The (effective) constructor method for the Error struct.
    pub fn new(kind: UserReqErrorKind, message: &'static str) -> Self {
        Self { kind, message }
    }
}