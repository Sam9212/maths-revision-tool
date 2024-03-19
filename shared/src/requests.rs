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

use serde::{Deserialize, Serialize};
use std::{error::Error, fmt::Display};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct UserReqError {
    kind: UserReqErrorKind,
    pub message: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum UserReqErrorKind {
    InvalidDetails,
    AccountLocked,
    ConnectionError,
    SerdeError,
    AddUserError,
    AddSetError,
    StrikeAddError,
    StrikeResetError,
    DeleteUserError,
    FetchUsersError,
    FetchQuestionsError,
    DeleteQuestionsError,
}

impl Display for UserReqError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{:?}] {}", self.kind, self.message)
    }
}

impl Error for UserReqError {}

impl From<String> for UserReqError {
    fn from(value: String) -> Self {
        let obj = &value[15..value.len() - 2];
        serde_json::from_str(obj).unwrap()
    }
}

impl UserReqError {
    /// The (effective) constructor method for the Error struct.
    pub fn new(kind: UserReqErrorKind, message: String) -> Self {
        Self { kind, message }
    }
}
