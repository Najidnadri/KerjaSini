extern crate argon2;

use std::{sync::mpsc, time::Duration};
use argon2::Config;
use password_hash::{self, SaltString};
use rand_core::OsRng;
use bytes::Bytes;
use eframe::epi::App;
use openssl::rsa::{Rsa, Padding};
use reqwest::{self, blocking::Response};
use serde::{Deserialize, Serialize};
use crate::{mainlogin::{self, EmployeeLoginCreds, EmployerLoginCreds}, signup::{self, EmployerSignupInfo, EmployeeSignupInfo}};

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

pub fn send_salt_request(phonenumber: String) -> ServerResponse {
    //encrypt body
    let mut buffer = Buffer {bytes: vec![]};
    let rsa = Rsa::public_key_from_pem(PUBLIC_KEY.as_bytes()).unwrap();
    let mut buf: Vec<u8> = vec![0; rsa.size() as usize];
    let _ = rsa.public_encrypt(phonenumber.as_bytes(), &mut buf, Padding::PKCS1).unwrap();
    buffer.bytes.push(buf);

    let client = reqwest::blocking::Client::new();
    let url = format!("{}salt", BASE_URL);
    let res: ServerResponse = client.post(url).json(&buffer).send().unwrap().json().unwrap();
    res
}

pub fn send_request(request: ClientRequest) -> ServerResponse {

    match request {
        ClientRequest::EmployeeLogin(mut creds) => {
            //request salt
            let mut salt = String::new();
            match send_salt_request(creds.phonenumber.clone()) {
                ServerResponse::SaltReceived(s) => {
                    salt = s;
                }
                _ => return ServerResponse::Err
            }
            //pass hashing
            let hash = hash_pass(&creds.pass, &salt);
            creds.pass = hash;

            //serialize
            let serialized_data = serde_json::to_string(&creds).unwrap();

            //encrypt
            let bytes = encrypt_data(serialized_data);

            //post request
            let response = post_request(bytes, "employeelogin");
            match response {
                ServerResponse::Ok => {
                    return ServerResponse::Ok;
                },
                _ => {
                    return ServerResponse::Err;
                }
            }
        },
        ClientRequest::EmployerLogin(creds) => todo!(),
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
            match response {
                ServerResponse::Ok => {
                    return ServerResponse::Ok;
                },
                _ => {
                    return ServerResponse::Err;
                }
            }
        },
        ClientRequest::EmployerSignup(info) => todo!(),
    }
    ServerResponse::Err
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