use serde::{Deserialize, Serialize};


#[derive(Debug, Deserialize, Serialize)]
pub struct EmployeeSignupInfo {
    pub phonenumber: String,
    pub fullname: String,
    pub username: String,
    pub email: String,
    pub age: String,
    pub pass: String,
    pub retype_pass: String,
    pub pass_visible: bool,
    pub postcode: String,
}


#[derive(Debug, Deserialize, Serialize)]
pub struct EmployerSignupInfo {
    pub phonenumber: String,
    pub fullname: String,
    pub companyname: String,
    pub email: String,
    pub website: String,
    pub regnum: String,
    pub pass: String,
    pub retype_pass: String,
    pub pass_visible: bool,
    pub postcode: String,
}


#[derive(Debug, Deserialize, Serialize)]
pub struct EmployerLoginCreds {
    pub name: String,
    pub pass: String,
    pub pass_visible: bool
}

#[derive(Debug, Deserialize, Serialize)]
pub struct EmployeeLoginCreds {
    pub name: String,
    pub pass: String,
    pub pass_visible: bool,
}
