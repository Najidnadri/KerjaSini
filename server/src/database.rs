use crate::login_signup::{EmployeeSignupInfo, EmployerSignupInfo, EmployeeLoginCreds};



pub fn query_employee_signup(data: EmployeeSignupInfo) -> String {
    format!("INSERT INTO Employee (
        PhoneNumber,
        Fullname,
        Username,
        Email,
        Age,
        Pass,
        Salt,
        Postcode ) VALUES ('{}', '{}', '{}', '{}', '{}', '{}', '{}', '{}');", 
    data.phonenumber, 
    data.fullname, 
    data.username,
    data.email,    
    data.age,  
    data.pass,
    data.salt,
    data.postcode)
}

pub fn query_employer_signup(data: EmployerSignupInfo) -> String {
    let _params = vec![
        &data.phonenumber, 
        &data.fullname, 
        &data.companyname,
        &data.email,    
        &data.website,  
        &data.regnum,
        &data.pass,
        &data.postcode
    ];
    format!("INSERT INTO Employer (
        PhoneNumber,
        Fullname,
        CompanyName,
        Email,
        Website.
        Regnum,
        Pass,
        Salt,
        Postcode ) VALUES ('{}', '{}', '{}', '{}', '{}', '{}', '{}', '{}', '{}');", 
    data.phonenumber, 
    data.fullname, 
    data.companyname,
    data.email,    
    data.website,  
    data.regnum,
    data.pass,
    data.salt,
    data.postcode)
}

pub fn query_employee_login() -> String {
    "SELECT * FROM Employee
    WHERE PhoneNumber = @P1;".to_string()
}


pub fn query_employer_login() -> String {
    "SELECT * FROM Employee
    WHERE PhoneNumber = @P1;".to_string()
}

pub fn query_all_employee() -> String {
    "SELECT * FROM Employee".to_string()
}

pub fn query_salt() -> String {
    "SELECT Salt FROM Employee
    WHERE PhoneNumber = @p1".to_string()
}

pub fn _sqli_checker(_params: Vec<&str>) -> bool {
    false
    //todo
    //implement libinjection crate later
    //as the crate now have some problem compiling it
}

