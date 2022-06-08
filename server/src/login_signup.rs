use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub enum PassStatus {
    TooShort,
    UniqueCharMissing,
    NumberMissing,
    UppercaseMissing,
    Good,
    Zero,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct EmployeeSignupInfo {
    pub phonenumber: String,
    pub fullname: String,
    pub username: String,
    pub email: String,
    pub age: String,
    pub pass: String,
    pub salt: String,
    pub retype_pass: String,
    pub pass_visible: bool,
    pub postcode: String,
    pub pass_status: PassStatus,
    pub retype_pass_err: bool,
    pub email_taken: bool,
    pub phonenumber_taken: bool,
    pub signup_button: bool,
}


#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct EmployerSignupInfo {
    pub phonenumber: String,
    pub fullname: String,
    pub companyname: String,
    pub email: String,
    pub website: String,
    pub regnum: String,
    pub pass: String,
    pub salt: String,
    pub retype_pass: String,
    pub pass_visible: bool,
    pub postcode: String,
    pub pass_status: PassStatus,
    pub retype_pass_err: bool,
    pub email_taken: bool,
    pub phonenumber_taken: bool,
    pub companyname_taken: bool,
    pub signup_button: bool,
}


#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct EmployerLoginCreds {
    pub phonenumber: String,
    pub pass: String,
    pub pass_visible: bool,
    pub err: bool,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct EmployeeLoginCreds {
    pub phonenumber: String,
    pub pass: String,
    pub pass_visible: bool,
    pub err: bool,
}
