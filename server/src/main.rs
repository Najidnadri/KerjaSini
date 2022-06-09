mod login_signup;
mod handler;
mod database;
mod error;

use actix_web::{post, Responder, HttpServer, App, web, get};
use bb8_tiberius::ConnectionManager;
use error::Error;
use serde::{Deserialize, Serialize};
//use openssl::{ssl::{SslAcceptor, SslFiletype, SslMethod}};
use tiberius::Config;
use bb8::{self, Pool};
use crate::{login_signup::{EmployeeSignupInfo, EmployerSignupInfo, EmployeeLoginCreds, EmployerLoginCreds}, handler::{decrypt_body, ServerResponse, check_creds_exist_employer, check_creds_exist_employee}, database::{query_employee_signup, query_employer_signup, query_employee_login, query_employer_login, query_employer_salt, query_employee_salt, query_and_get_first_row}};

type Dbpool = bb8::Pool<ConnectionManager>;

#[derive(Debug, Deserialize, Serialize)]
pub struct Buffer {
    pub bytes: Vec<Vec<u8>>
}

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
        .service(get_employee_salt)
        .service(get_employer_salt)
        .service(employer_signup)
        .service(employer_login)
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
async fn employee_signup(body: web::Json<Buffer>, pool: web::Data<Dbpool>) -> impl Responder {
    //get connection with database
    let conn = pool.get().await;
    if let Err(e) = &conn {
        println!("error getting conn from pool main::employee_signup: {:?}", e);
        return web::Json(ServerResponse::ServerFailed)
    }
    let mut conn = conn.unwrap();

    //decrypt data
    let decrypt_data = decrypt_body(body.into_inner()).await;
    println!("{}", decrypt_data);

    //deserial data
    let deserial_data: Result<EmployeeSignupInfo, serde_json::error::Error> = serde_json::from_str(decrypt_data.trim());
    if let Err(e) = &deserial_data {
        println!("main::employee_signup {:?}", e);
        return web::Json(ServerResponse::ServerFailed);
    }
    let deserial_data = deserial_data.unwrap();

    //check email, phone number, company name
    let creds_check_res = check_creds_exist_employee(&mut conn, &deserial_data.phonenumber, &deserial_data.email).await;
    if let Err(_e) = creds_check_res {
        return web::Json(ServerResponse::ServerFailed)
    }
    let creds_check_res = creds_check_res.unwrap();
    if let Some(signuperr) = creds_check_res {
        return web::Json(ServerResponse::SignupErr(signuperr))
    }
    drop(creds_check_res);

    //upload to database
    let query = query_employee_signup(deserial_data);
    let res = conn.execute(&query, &[]).await;
    if let Err(e) = &res {
        println!("error executing query main::employee_signup {:?}", e);
        return web::Json(ServerResponse::ServerFailed)
    }
    println!("employee signed up: {}", query);
    
    web::Json(ServerResponse::Ok)
}

#[post("/employersignup")]
async fn employer_signup(body: web::Json<Buffer>, pool: web::Data<Dbpool>) -> impl Responder {
    //get connection with database
    let conn = pool.get().await;
    if let Err(e) = &conn {
        println!("error getting conn from pool main::employee_signup: {:?}", e);
        return web::Json(ServerResponse::ServerFailed)
    }
    let mut conn = conn.unwrap();

    //decrypt data
    let decrypt_data = decrypt_body(body.into_inner()).await;
    println!("{}", decrypt_data);

    //deserial data
    let deserial_data: Result<EmployerSignupInfo, serde_json::error::Error> = serde_json::from_str(decrypt_data.trim());
    if let Err(e) = &deserial_data {
        println!("main::employersignupinfo {:?}", e);
        return web::Json(ServerResponse::ServerFailed)
    }
    let deserial_data = deserial_data.unwrap();

    //check email, phone number, company name
    let creds_check_res = check_creds_exist_employer(&mut conn, &deserial_data.phonenumber, &deserial_data.email , &deserial_data.companyname).await;
    if let Err(_e) = creds_check_res {
        return web::Json(ServerResponse::ServerFailed)
    }
    let creds_check_res = creds_check_res.unwrap();
    if let Some(signuperr) = creds_check_res {
        return web::Json(ServerResponse::SignupErr(signuperr))
    }
    drop(creds_check_res);

    //upload to database
    let query = query_employer_signup(deserial_data);
    let res = conn.execute(&query, &[]).await;
    if let Err(e) = &res {
        println!("error executing query main::employer_signup {:?}", e);
        return web::Json(ServerResponse::ServerFailed)
    }
    println!("employer signed up: {}", query);
    
    web::Json(ServerResponse::Ok)
}

#[post("/employeelogin")]
async fn employee_login(body: web::Json<Buffer>, pool: web::Data<Dbpool>) -> impl Responder {
    //get connection with database
    let conn = pool.get().await;
    if let Err(e) = &conn {
        println!("error getting conn from pool main::employee_signup: {:?}", e);
        return web::Json(ServerResponse::ServerFailed)
    }
    let mut conn = conn.unwrap();

    //decrypt data
    let decrypt_data = decrypt_body(body.into_inner()).await;
    println!("{}", decrypt_data);

    //deserial data
    let deserial_data: Result<EmployeeLoginCreds, serde_json::error::Error> = serde_json::from_str(decrypt_data.trim());
    if let Err(e) = &deserial_data {
        println!("main::employee_login {:?}", e);
        return web::Json(ServerResponse::ServerFailed)
    }

    //create query to database
    let query = query_employee_login();

    //execute query and get first row
    let row = query_and_get_first_row(&mut conn, query, &[&deserial_data.as_ref().unwrap().phonenumber]).await;
    if let Err(ref e) = row {
        match e {
            Error::ServerError => return web::Json(ServerResponse::ServerFailed),
            Error::ZeroRow => return web::Json(ServerResponse::LoginErr),
        }
    }
    let row = row.unwrap();

    //get value from column "Pass"
    let pass: Option<&str> = row.get("Pass");
    if let None = pass {
        println!("Column does not have any value in it. Column: Pass, Row: {:?},", &row );
        return web::Json(ServerResponse::LoginErr);
    }

    //check pass
    if deserial_data.unwrap().pass.trim() == pass.unwrap().trim() {
        return web::Json(ServerResponse::Ok)
    } else {
        return web::Json(ServerResponse::LoginErr)
    }
}

#[post("/employerlogin")]   
async fn employer_login(body: web::Json<Buffer>, pool: web::Data<Dbpool>) -> impl Responder {
    //get connection with database
    let conn = pool.get().await;
    if let Err(e) = &conn {
        println!("error getting conn from pool main::employee_signup: {:?}", e);
        return web::Json(ServerResponse::ServerFailed)
    }
    let mut conn = conn.unwrap();

    //decrypt data
    let decrypt_data = decrypt_body(body.into_inner()).await;
    println!("{}", decrypt_data);

    //deserial data
    let deserial_data: Result<EmployerLoginCreds, serde_json::error::Error> = serde_json::from_str(decrypt_data.trim());
    if let Err(e) = &deserial_data {
        println!("main::employer_login {:?}", e);
        return web::Json(ServerResponse::ServerFailed)
    }

    //create query to database
    let query = query_employer_login();

    //execute query and get first row
    let row = query_and_get_first_row(&mut conn, query, &[&deserial_data.as_ref().unwrap().phonenumber]).await;
    if let Err(ref e) = row {
        match e {
            Error::ServerError => return web::Json(ServerResponse::ServerFailed),
            Error::ZeroRow => return web::Json(ServerResponse::LoginErr),
        }
    }
    let row = row.unwrap();

    //get value from column "Pass"
    let pass: Option<&str> = row.get("Pass");
    if let None = pass {
        println!("Column does not have any value in it. Column: Pass, Row: {:?},", &row );
        return web::Json(ServerResponse::LoginErr);
    }

    //check pass
    if deserial_data.unwrap().pass.trim() == pass.unwrap().trim() {
        return web::Json(ServerResponse::Ok)
    } else {
        return web::Json(ServerResponse::LoginErr)
    }
}

#[post("/employeesalt")]
async fn get_employee_salt(body: web::Json<Buffer>, pool: web::Data<Dbpool>) -> impl Responder {
    //get connection with database
    let conn = pool.get().await;
    if let Err(e) = &conn {
        println!("error getting conn from pool main::employee_signup: {:?}", e);
        return web::Json(ServerResponse::ServerFailed)
    }
    let mut conn = conn.unwrap();

    //decrypt data
    let decrypt_data = decrypt_body(body.into_inner()).await;
    
    //create query
    let query = query_employee_salt();

    //execute query and get first row
    let row = query_and_get_first_row(&mut conn, query, &[&decrypt_data]).await;
    if let Err(ref e) = row {
        match e {
            Error::ServerError => return web::Json(ServerResponse::ServerFailed),
            Error::ZeroRow => return web::Json(ServerResponse::LoginErr),
        }
    }
    let row = row.unwrap();

    //get value from column "Salt"
    let salt: Option<&str> = row.get("Salt");
    if let None = salt {
        println!("Column does not have any value in it. Column: Salt, Row: {:?},", &row );
        return web::Json(ServerResponse::LoginErr);
    }

    //create server response
    let server_response = ServerResponse::SaltReceived(salt.unwrap().to_string());

    web::Json(server_response)
}

#[post("/employersalt")]
async fn get_employer_salt(body: web::Json<Buffer>, pool: web::Data<Dbpool>) -> impl Responder {
    //get connection to database
    let conn = pool.get().await;
    if let Err(e) = &conn {
        println!("error getting conn from pool main::employer_salt: {:?}", e);
        return web::Json(ServerResponse::ServerFailed)
    }
    let mut conn = conn.unwrap();

    //decrypt data
    let decrypt_data = decrypt_body(body.into_inner()).await;
    
    //create query
    let query = query_employer_salt();

    //execute query and get first row
    let row = query_and_get_first_row(&mut conn, query, &[&decrypt_data]).await;
    if let Err(ref e) = row {
        match e {
            Error::ServerError => return web::Json(ServerResponse::ServerFailed),
            Error::ZeroRow => return web::Json(ServerResponse::LoginErr),
        }
    }
    let row = row.unwrap();

    //get value from column "Salt"
    let salt: Option<&str> = row.get("Salt");
    if let None = salt {
        println!("Column does not have any value in it. Column: Salt, Row: {:?},", &row );
        return web::Json(ServerResponse::LoginErr);
    }

    //create server response
    let server_response = ServerResponse::SaltReceived(salt.unwrap().to_string());

    web::Json(server_response)
}
