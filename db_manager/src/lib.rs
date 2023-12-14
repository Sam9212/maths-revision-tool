use chrono::NaiveDate;
use serde::{Serialize, Deserialize};

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
    pub fn new(username: String, password: String, date_of_birth: String, access_level: AccessLevel) -> Self {
        let date_of_birth = NaiveDate::parse_from_str(&date_of_birth, "%Y-%m-%d").unwrap();
        Self { username, password, date_of_birth, access_level, strikes: 0 }
    }

    pub fn username(&self) -> &String {
        &self.username
    }

    pub fn password(&self) -> &String {
        &self.password
    }

    pub fn strikes(&self) -> u8 {
        self.strikes
    }

    pub fn add_strike(&mut self) {
        self.strikes += 1;
    }

    pub fn remove_strikes(&mut self) {
        self.strikes = 0;
    }
}