
//! This module has structs which are used by my frontend to 
//! call the backend commands. Each struct needs to be Serializeable, 
//! so that it can be transported from the frontend the backend, and
//! each of them contains an attribute for each parameter of the
//! corresponding function that they are used in.

use serde::Serialize;

#[allow(non_snake_case)]
#[derive(Serialize)]
pub struct AddUserArgs {
    pub newUser: crate::User,
}


#[derive(Serialize)]
pub struct ValidateLoginArgs {
    pub username: String,
    pub password: String,
}