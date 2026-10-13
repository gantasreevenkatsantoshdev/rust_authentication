
use chrono::{DateTime, Utc};
use core::str;
use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::models::{User, UserRole};

#[derive(Debug, Validate, Default, Clone, Serialize, Deserialize)]
pub struct RegisterUserDto {
    #[validate(length(min = 1, message = "Name is required"))]
    pub name: String,
    #[validate(
        length(min = 1, message = "Email is required"),
        email(message = "Email is invalid")
    )]
    pub email: String,
    #[validate(
        length(min = 1, message = "Password is Required"),
        length(min = 6, message = "Password must be at least 6 character")
    )]
    pub password: String,
    #[validate(
        length(min = 1, message = "Confirm password is required"),
        must_match(other = "password", message = "Password do not match")
    )]
    #[serde(rename = "Password_confirm")]
    pub password_confirm: String,
}

#[derive(Debug, Validate, Default, Clone, Deserialize, Serialize)]
pub struct LoginUserDto {
    #[validate(
        length(min = 1, message = "Email is required"),
        email(message = "Email is invalid")
    )]
    pub email: String,
    #[validate(
        length(min = 1, message = "Password is required"),
        length(min = 6, message = "Password must be of 6 character")
    )]
    pub password: String,
}
