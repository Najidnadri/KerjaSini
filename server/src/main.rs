mod login_signup;
mod handler;
mod database;

use actix_web::{post, Responder, HttpServer, App, HttpResponse, web, get};
use bb8_tiberius::ConnectionManager;
use database::query_all_employee;
//use openssl::{ssl::{SslAcceptor, SslFiletype, SslMethod}};
use tiberius::{Config, FromSqlOwned};
use bb8::{self, Pool};
use crate::{login_signup::{EmployeeSignupInfo, EmployerSignupInfo, EmployeeLoginCreds, EmployerLoginCreds}, handler::decrypt_body, database::{query_employee_signup, query_employer_signup, query_employee_login, query_employer_login}};

type Dbpool = bb8::Pool<ConnectionManager>;

#[actix_web::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    //create connection pool
    //config
    let mut config = Config::new();
    config.host("localhost");
    config.port(1433);
    config.authentication(tiberius::AuthMethod::sql_server("sa", "Muhd_najid01"));
    config.trust_cert();
    config.database("kerjasini");

    //manager
    let mgr = bb8_tiberius::ConnectionManager::build(config).unwrap();

    //pool
    let pool = Pool::builder().max_size(10).build(mgr).await.unwrap();


    //run up the server
    println!("actix web go!");
    /* 
    let mut builder = SslAcceptor::mozilla_intermediate(SslMethod::tls()).unwrap();
    builder
        .set_private_key_file("key.pem", SslFiletype::PEM)
        .unwrap();
    builder.set_certificate_chain_file("cert.pem").unwrap();
    */

    HttpServer::new(move || {
        App::new()
        .app_data(web::Data::new(pool.clone()))
        .service(employee_signup)
        .service(index)
        .service(employee_login)
    })
    .bind(("127.0.0.1", 8000))?
    //.bind_openssl("127.0.0.1:8000", builder)? //bind for ssl
    .workers(4)
    .run()
    .await
    .unwrap();

    Ok(())
}

#[get("/")]
async fn index() -> impl Responder {
    
    "hello world"
}

#[post("/employeesignup")]
async fn employee_signup(body: web::Bytes, pool: web::Data<Dbpool>) -> impl Responder {
    //decrypt data
    let decrypt_data = decrypt_body(body.to_vec()).await;
    println!("{}", decrypt_data);

    //deserial data
    let deserial_data: EmployeeSignupInfo = serde_json::from_str(decrypt_data.trim()).expect("cannot deserial employeesignupinfo");

    //upload to database
    let query = query_employee_signup(deserial_data);
    let mut conn = pool.get().await.expect("err in getting conn main::employee_signup");
    let _res = conn.execute(&query, &[]).await.expect("error in executing query main::employee_signup");
    println!("employee signed up: {}", query);
    HttpResponse::Ok()
}

#[post("/employersignup")]
async fn employer_signup(body: web::Bytes, pool: web::Data<Dbpool>) -> impl Responder {
    //decrypt data
    let decrypt_data = decrypt_body(body.to_vec()).await;
    println!("{}", decrypt_data);

    //deserial data
    let deserial_data: EmployerSignupInfo = serde_json::from_str(decrypt_data.trim()).expect("cannot deserial employersignupinfo");

    //upload to database
    let query = query_employer_signup(deserial_data);
    let mut conn = pool.get().await.expect("err in getting conn main::employer_signup");
    let _res = conn.execute(&query, &[]).await.expect("error in executing query main::employer_signup");
    println!("employer signed up: {}", query);
    HttpResponse::Ok()
}

#[post("/employeelogin")]
async fn employee_login(body: web::Bytes, pool: web::Data<Dbpool>) -> impl Responder {
    //decrypt data
    let decrypt_data = decrypt_body(body.to_vec()).await;
    println!("{}", decrypt_data);

    //deserial data
    let deserial_data: EmployeeLoginCreds = serde_json::from_str(decrypt_data.trim()).expect("cannot deserial employersignupinfo");

    //create query to database
    let query = query_employee_login();
    let mut conn = pool.get().await.expect("err in getting conn main::employer_signup");

    //response from database
    let response = conn.query(&query, &[&deserial_data.phonenumber])
    .await
    .expect("err in executing query main::employee_login")
    .into_row()
    .await
    .expect("error into_first_result");

    //response.remove(0);
    for val in response.unwrap().into_iter() {
        let string = String::from_sql_owned(val).unwrap().unwrap();
        println!("{}", string);
    }

    HttpResponse::Ok()
}

#[post("/employerlogin")]   
async fn employer_login(body: web::Bytes, pool: web::Data<Dbpool>) -> impl Responder {
    //decrypt data
    let decrypt_data = decrypt_body(body.to_vec()).await;
    println!("{}", decrypt_data);

    //deserial data
    let deserial_data: EmployerLoginCreds = serde_json::from_str(decrypt_data.trim()).expect("cannot deserial employersignupinfo");

    //create query to database
    let query = query_employer_login();
    let mut conn = pool.get().await.expect("err in getting conn main::employer_signup");

    //response from database
    let response = conn.query(&query, &[&deserial_data.phonenumber])
    .await
    .expect("err in executing query main::employee_login")
    .into_row()
    .await
    .expect("error into_first_result");

    //iter through the rows;
    for val in response.unwrap().into_iter() {
        let string = String::from_sql_owned(val).unwrap().unwrap();
        println!("{}", string);
    }
    HttpResponse::Ok()
}

#[get("/allemployee")]
async fn all_employee(pool: web::Data<Dbpool>) -> impl Responder {
    //create query to database
    let query = query_all_employee();
    let mut conn = pool.get().await.expect("err in getting conn main::all_employee");

    //response from database
    let response = conn.query(&query, &[])
    .await
    .expect("err in executing query main::all_employee")
    .into_row()
    .await
    .expect("err into_row main::all_employee");

    //do something with data
    
    HttpResponse::Ok()
}
