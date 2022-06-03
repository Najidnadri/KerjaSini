mod login_signup;

use actix_web::{post, Responder, HttpServer, App, HttpResponse, web};
use crate::login_signup::{EmployeeSignupInfo, EmployerSignupInfo, EmployeeLoginCreds, EmployerLoginCreds};

#[actix_web::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    //set up demo data


    //run up the server
    println!("actix web go!");

    HttpServer::new(|| {
        App::new()
        .service(employee_signup)
    })
    .bind(("127.0.0.1", 8000))?
    .workers(4)
    .run()
    .await
    .unwrap();

    Ok(())
}

#[post("/employeesignup")]
async fn employee_signup(body: web::Json<EmployeeSignupInfo>) -> impl Responder {
    println!("{:?}", body);
    HttpResponse::Ok()
}

#[post("/employersignup")]
async fn employer_signup(body: web::Json<EmployerSignupInfo>) -> impl Responder {
    HttpResponse::Ok()
}

#[post("/employeelogin")]
async fn employee_login(body: web::Json<EmployeeLoginCreds>) -> impl Responder {
    HttpResponse::Ok()
}

#[post("/employerlogin")]
async fn employer_login(body: web::Json<EmployerLoginCreds>) -> impl Responder {
    HttpResponse::Ok()
}
