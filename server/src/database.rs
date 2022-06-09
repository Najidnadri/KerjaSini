use bb8::PooledConnection;
use bb8_tiberius::ConnectionManager;
use tiberius::{Row, ToSql};

use crate::{login_signup::{EmployeeSignupInfo, EmployerSignupInfo}, error::Error};



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
        Website,
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
    "SELECT * FROM Employer
    WHERE PhoneNumber = @P1;".to_string()
}

pub fn _query_all_employee() -> String {
    "SELECT * FROM Employee".to_string()
}

pub fn query_employee_salt() -> String {
    "SELECT Salt FROM Employee
    WHERE PhoneNumber = @p1".to_string()
}

pub fn query_employer_salt() -> String {
    "SELECT Salt FROM Employer
    WHERE PhoneNumber = @p1".to_string()
}

pub fn query_phone_number_employee() -> String {
    "SELECT PhoneNumber FROM Employee
    WHERE PhoneNumber = @P1".to_string()
}

pub fn query_phone_number_employer() -> String {
    "SELECT PhoneNumber FROM Employer
    WHERE PhoneNumber = @P1".to_string()
}

pub fn query_email_employer() -> String {
    "SELECT Email FROM Employer
    WHERE Email = @P1".to_string()
}

pub fn query_email_employee() -> String {
    "SELECT Email FROM Employee
    WHERE Email = @P1".to_string()
}

pub fn query_companyname() -> String {
    "SELECT Companyname FROM Employer
    WHERE Companyname = @P1".to_string()
}

pub async fn query_and_get_first_row(
    conn: &mut PooledConnection<'_, ConnectionManager>,
    query: String,
    params: &[&dyn ToSql],

) -> Result<Row, Error> {
    //execute query
    let response = conn.query(&query, params).await;
    if let Err(e) = &response {
        println!("error in executing query database::query_and_get_first_row | query : {} | error : {:?}", query, e);
        return Err(Error::ServerError)
    }

    //get first row
    let row_result = response.unwrap()
    .into_row()
    .await;
    if let Err(e) = &row_result {
        println!("error into_row() database::query_and_get_first_row | query : {} | error: {:?}", query, e);
        return Err(Error::ServerError)
    }
    if let None = &row_result.as_ref().unwrap() {
        println!("No content for given query | query = {:?}", &query);
        return Err(Error::ZeroRow)
    }
    Ok(row_result.unwrap().unwrap())
}

pub fn _sqli_checker(_params: Vec<&str>) -> bool {
    false
    //todo
    //implement libinjection crate later
    //as the crate now have some problem compiling it
}

