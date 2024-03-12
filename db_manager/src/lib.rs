pub mod commands;
pub mod requests;

use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

/// A struct which groups the information held in the database on every user,
/// and acts as a ser/de target to make the transition between database & client
/// more frictionless.
///
/// The derive macro used on this struct adds auto-implementations of frequently
/// used traits to the struct, such as an implementation of PartialEq and Eq, which
/// are traits that define how 2 instances of the User struct will be compared against
/// each other. The auto-implementation of Clone here makes a deep clone of the entire
/// struct and all its fields, which sometimes needs to be used, despite its relative
/// slowness, for convenience. These implementations are not anywhere near as important,
/// however, as the Serialize and Deserialize implementations. By deriving these traits on
/// my User struct, the struct now knows how to be correctly converted into a string of
/// text/bytes/binary/etc which can be deserialized back into a valid Rust type. This is
/// the core feature of my system as it allows me to directly insert data of the User type
/// into the database, and also allows me to instantly receive back User structs from the
/// database, as the function which sends/receives them will automatically call the Serialize
/// and Deserialize traits to convert them to and from a data type that can be stored in a
/// database.
#[derive(PartialEq, Eq, Clone, Serialize, Deserialize, Debug)]
pub struct User {
    username: String,
    password: String,
    date_of_birth: NaiveDate,
    access_level: AccessLevel,
    strikes: u8,
}

#[derive(PartialEq, Eq, Clone, Copy, Serialize, Deserialize, Debug)]
pub enum AccessLevel {
    USER,
    TEACHER,
    ADMIN,
}

impl User {
    /// The (effective) constructor for the [`User`] struct.
    ///
    /// This function simply takes in arguments to set the fields of a new instance of
    /// the class, returning that new instance to the caller. It does do a minor amount
    /// of computation to convert the DOB string that comes from the HTML `input` element
    /// into a [`NaiveDate`] which is an easily usable and Serializable data type.
    pub fn new(
        username: String,
        password: String,
        date_of_birth: String,
        access_level: AccessLevel,
    ) -> Self {
        let date_of_birth = NaiveDate::parse_from_str(&date_of_birth, "%Y-%m-%d").unwrap();
        Self {
            username,
            password,
            date_of_birth,
            access_level,
            strikes: 0,
        }
    }

    /// A simple ref-getter which returns a reference to the username held within
    /// this struct.
    pub fn username(&self) -> &String {
        &self.username
    }

    /// A simple ref-getter which returns a reference to the password held within
    /// this struct.
    pub fn password(&self) -> &String {
        &self.password
    }

    /// A simple getter which returns a copy of the strike count within
    /// this struct.
    pub fn strikes(&self) -> u8 {
        self.strikes
    }

    /// A simple ref-getter which returns a reference to the access level value
    /// held within this struct.
    pub fn access_level(&self) -> &AccessLevel {
        &self.access_level
    }
}
