mod login_signup;
mod handler;
mod database;

use actix_web::{post, Responder, HttpServer, App, HttpResponse, web, get};
use bb8_tiberius::ConnectionManager;
//use openssl::{ssl::{SslAcceptor, SslFiletype, SslMethod}};
use tiberius::Config;
use bb8::{self, Pool};
use crate::{login_signup::{EmployeeSignupInfo, EmployerSignupInfo, EmployeeLoginCreds, EmployerLoginCreds}, handler::decrypt_body};

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
    println!("{:?}", body);
    //decrypt data
    let decrypt_data = decrypt_body(body.to_vec()).await;
    println!("{}", decrypt_data);

    //deserial data
    let deserial_data: EmployeeSignupInfo = serde_json::from_str(decrypt_data.trim()).expect("cannot deserial employeesignupinfo");

    //upload to database
    let query = format!("INSERT INTO Employee (
        PhoneNumber,
        Fullname,
        Username,
        Email,
        Age,
        Pass,
        Postcode ) VALUES ('{}', '{}', '{}', '{}', '{}', '{}', '{}');", 
    deserial_data.phonenumber, 
    deserial_data.fullname, 
    deserial_data.username,
    deserial_data.email,    
    deserial_data.age,  
    deserial_data.pass,
    deserial_data.postcode);
    let mut conn = pool.get().await.expect("err in getting conn main::employee_signup");
    let _res = conn.execute(&query, &[]).await.expect("error in executing query main::employee_signup");
    println!("employee signed up: {}", query);
    HttpResponse::Ok()
}

#[post("/employersignup")]
async fn employer_signup(_body: web::Json<EmployerSignupInfo>) -> impl Responder {
    HttpResponse::Ok()
}

#[post("/employeelogin")]
async fn employee_login(_body: web::Json<EmployeeLoginCreds>) -> impl Responder {
    HttpResponse::Ok()
}

#[post("/employerlogin")]
async fn employer_login(_body: web::Json<EmployerLoginCreds>) -> impl Responder {
    HttpResponse::Ok()
}
