extern crate argon2;

use argon2::Config;
use password_hash::{self, SaltString};
use rand_core::OsRng;
use eframe::epi::App;
use openssl::rsa::{Rsa, Padding};
use reqwest;
use serde::{Deserialize, Serialize};
use crate::{mainlogin::{self, EmployeeLoginCreds, EmployerLoginCreds}, signup::{self, EmployerSignupInfo, EmployeeSignupInfo, SignupErr}};

const BASE_URL: &str = "http://127.0.0.1:8000/";

const PUBLIC_KEY: &str = "-----BEGIN PUBLIC KEY-----
MIIBIjANBgkqhkiG9w0BAQEFAAOCAQ8AMIIBCgKCAQEAptz5xhCrAnPqd/vCvVcT
bbJeshZWdDLtDQG0yggpoqfsC+UGZyBeyfXsF6aNxs4hEzUVcrhFBwgjgrGWxgBs
0pQaAVQ3UY5ynFDrH4gKKbK2Q+zeZLHrNKdOA7ZrquW2yjsOoIxDzxGh7nRyIfZj
JixGf5J2vWMLet7ntvGUfmnSQkI8N8TTFA/tKbruEqYPKBe5VV5xb28dSEpyzef8
E8yYutJuHs5s7hHksOaVeiGnmIxK1FqK5yHeDm/yaWGS6+WDxrJRbDmgS+WDDc6R
geOOYIqpZZWSuQBMnUdqO2mX6pRdRcvno/mtkdKe30zbaE5Hf7A/WyBAeJFn+Dv8
nQIDAQAB
-----END PUBLIC KEY-----";

#[derive(Debug, Deserialize, Serialize)]
struct Buffer {
    bytes: Vec<Vec<u8>>
}

pub enum ClientRequest {
    EmployeeLogin(EmployeeLoginCreds),
    EmployerLogin(EmployerLoginCreds),
    EmployeeSignup(EmployeeSignupInfo),
    EmployerSignup(EmployerSignupInfo),
}

#[derive(Debug, Deserialize, Serialize)]
pub enum ServerResponse {
    SaltReceived(String),
    SignupErr(SignupErr),
    LoginErr,
    ServerFailed,
    Ok,
    Err,
}

pub struct Event {
    pub data: Data,
    pub page: Page
}

pub struct Data {
    pub employee_login: EmployeeLoginCreds,
    pub employer_login: EmployerLoginCreds,
    pub employer_signup: EmployerSignupInfo,
    pub employee_signup: EmployeeSignupInfo,
}

impl Data {
    pub fn new() -> Self {
        Data { employee_login: EmployeeLoginCreds::new(), employer_login: EmployerLoginCreds::new(), employer_signup: EmployerSignupInfo::new(), employee_signup: EmployeeSignupInfo::new() }
    }
}

pub enum Page {
    MainLogin,
    EmployeeLogin,
    EmployerLogin,
    EmployeeSignup,
    EmployerSignup,
}

impl App for Event {
    fn update(&mut self, ctx: &egui::CtxRef, _frame: &mut eframe::epi::Frame<'_>) {
        match self.page {
            Page::MainLogin => mainlogin::main_login(self, ctx),
            Page::EmployeeLogin => mainlogin::employee_login(self, ctx),
            Page::EmployerLogin => mainlogin::employer_login(self, ctx),
            Page::EmployeeSignup => signup::employee_signup(self, ctx),
            Page::EmployerSignup=> signup::employer_signup(self, ctx),
        }
    }

    fn name(&self) -> &str {
        "KerjaSini"
    }
}

pub fn send_salt_request_employee(phonenumber: String) -> ServerResponse {
    //encrypt body
    let mut buffer = Buffer {bytes: vec![]};
    let rsa = Rsa::public_key_from_pem(PUBLIC_KEY.as_bytes()).unwrap();
    let mut buf: Vec<u8> = vec![0; rsa.size() as usize];
    let _ = rsa.public_encrypt(phonenumber.as_bytes(), &mut buf, Padding::PKCS1).unwrap();
    buffer.bytes.push(buf);

    let client = reqwest::blocking::Client::new();
    let url = format!("{}employeesalt", BASE_URL);
    let res: ServerResponse = client.post(url).json(&buffer).send().unwrap().json().unwrap();
    res
}

pub fn send_salt_request_employer(phonenumber: String) -> ServerResponse {
    //encrypt body
    let mut buffer = Buffer {bytes: vec![]};
    let rsa = Rsa::public_key_from_pem(PUBLIC_KEY.as_bytes()).unwrap();
    let mut buf: Vec<u8> = vec![0; rsa.size() as usize];
    let _ = rsa.public_encrypt(phonenumber.as_bytes(), &mut buf, Padding::PKCS1).unwrap();
    buffer.bytes.push(buf);

    let client = reqwest::blocking::Client::new();
    let url = format!("{}employersalt", BASE_URL);
    let res: ServerResponse = client.post(url).json(&buffer).send().unwrap().json().unwrap();
    res
}

pub fn send_request(request: ClientRequest) -> ServerResponse {

    match request {
        ClientRequest::EmployeeLogin(mut creds) => {
            //request salt
            let mut _salt = String::new();
            let salt_response = send_salt_request_employee(creds.phonenumber.clone());
            match salt_response {
                ServerResponse::SaltReceived(s) => {
                    _salt = s;
                },
                _ => return salt_response
            }
            //pass hashing
            let hash = hash_pass(&creds.pass, &_salt);
            creds.pass = hash;

            //serialize
            let serialized_data = serde_json::to_string(&creds).unwrap();

            //encrypt
            let bytes = encrypt_data(serialized_data);

            //post request
            let response = post_request(bytes, "employeelogin");
            return response
        },
        ClientRequest::EmployerLogin(mut creds) => {
            //request salt
            let mut _salt = String::new();
            let salt_response = send_salt_request_employer(creds.phonenumber.clone());
            match salt_response {
                ServerResponse::SaltReceived(s) => {
                    _salt = s;
                },
                _ => return salt_response
            }
            //pass hashing
            let hash = hash_pass(&creds.pass, &_salt);
            creds.pass = hash;

            //serialize
            let serialized_data = serde_json::to_string(&creds).unwrap();

            //encrypt
            let bytes = encrypt_data(serialized_data);

            //post request
            let response = post_request(bytes, "employerlogin");
            return response
        },
        ClientRequest::EmployeeSignup(mut info) => {
            //generate salt
            let salt = SaltString::generate(&mut OsRng).to_string();

            //pass hashing
            let hash = hash_pass(&info.pass, &salt);
            info.pass = hash;
            info.salt = salt;

            //serialize
            let serialized_data = serde_json::to_string(&info).unwrap();

            //encrypt
            let bytes = encrypt_data(serialized_data);

            //post request
            let response = post_request(bytes, "employeesignup");
            return response
        },
        ClientRequest::EmployerSignup(mut info) => {
             //generate salt
            let salt = SaltString::generate(&mut OsRng).to_string();

            //pass hashing
            let hash = hash_pass(&info.pass, &salt);
            info.pass = hash;
            info.salt = salt;

            //serialize
            let serialized_data = serde_json::to_string(&info).unwrap();

            //encrypt
            let bytes = encrypt_data(serialized_data);

            //post request
            let response = post_request(bytes, "employersignup");
            return response
        },
    }
    //ServerResponse::Err
}

fn hash_pass(pass: &str, salt: &str) -> String {
    let config = Config::default();
    let hash =  argon2::hash_encoded(pass.as_bytes(), salt.as_bytes(), &config).unwrap();
    hash
}

fn encrypt_data(data: String) -> Buffer {
    println!("got into encrypt data");
    //divide into smaller parts
    let data_chunks = data
    .as_bytes()
    .chunks(128)
    .map(|buf| unsafe { std::str::from_utf8_unchecked(buf) })
    .collect::<Vec<&str>>();

    //encrypt each chunk and add into buffer struct
    let mut buffer = Buffer {
        bytes: vec![],
    };
    for str in data_chunks {
        let rsa = Rsa::public_key_from_pem(PUBLIC_KEY.as_bytes()).unwrap();
        let mut buf: Vec<u8> = vec![0; rsa.size() as usize];
        let _ = rsa.public_encrypt(str.as_bytes(), &mut buf, Padding::PKCS1).unwrap();
        buffer.bytes.push(buf);
    }
    println!("ecnrypted data");
    buffer
}

fn post_request(body: Buffer, route: &str) -> ServerResponse {
    let url = format!("{}{}", BASE_URL, route);
    let client = reqwest::blocking::Client::new();
    let res = client.post(url)
    .json(&body)
    .send()
    .unwrap();

    let server_response: Result<ServerResponse, _> = res.json();
    match server_response {
        Ok(response) => {
            return response
        },
        Err(_) => {
            return ServerResponse::ServerFailed
        }
    }
}

pub fn filter_response(event: &mut Event, response: ServerResponse) {
    match response {
        ServerResponse::SignupErr(err) => {
            match event.page {
                Page::EmployeeSignup => {
                    event.data.employee_signup.email_taken = err.email_taken;
                    event.data.employee_signup.phonenumber_taken = err.phone_number_taken;
                },
                Page::EmployerSignup => {
                    event.data.employer_signup.email_taken = err.email_taken;
                    event.data.employer_signup.phonenumber_taken = err.phone_number_taken;
                    event.data.employer_signup.companyname_taken = err.company_name_taken;
                },
                _ => (),
            }
        },
        ServerResponse::LoginErr => {
            match event.page {
                Page::EmployeeLogin => {
                    event.data.employee_login.err = true;
                },
                Page::EmployerLogin => {
                    event.data.employer_login.err = true;
                }
                _ => (),
            }
        },
        ServerResponse::ServerFailed => todo!(),
        ServerResponse::Ok => {
            match event.page {
                Page::EmployeeLogin => {
                    event.data.employee_login.clear();
                    event.page = Page::MainLogin;
                },
                Page::EmployerLogin => {
                    event.data.employer_login.clear();
                    event.page = Page::MainLogin;
                },
                Page::EmployeeSignup => {
                    event.data.employee_signup.clear();
                    event.page = Page::EmployeeLogin;
                },
                Page::EmployerSignup => {
                    event.data.employer_signup.clear();
                    event.page = Page::EmployerLogin;
                },
                _ => (),
            }
        },
        ServerResponse::Err => todo!(),
        //saltreceived
        _ => todo!(), 
    }
}