use bb8::PooledConnection;
use bb8_tiberius::ConnectionManager;
use openssl::rsa::{Rsa, Padding};
use serde::{Deserialize, Serialize};

use crate::{Buffer, database::{query_phone_number_employee, query_email_employee, query_phone_number_employer, query_email_employer, query_companyname}};


const PRIVATE_KEY: &str = "-----BEGIN RSA PRIVATE KEY-----
Proc-Type: 4,ENCRYPTED
DEK-Info: AES-128-CBC,E0C62636CD26D3F2F7182A28CDA84555

7d6gA0AazAUhYFBDoB6Je7OTsULcCaBkm9ccnjGa7JtfcJGTGBOnG0nJ5qsqx/gj
TYrGhi6/Z47Nc8L0ilIz3AGcZum/QfIIhxr1FNFebqPFyj2Oxl+RtTrDs+lTfbO3
iuxQB4oqM4+Qabqq87xhpTJ3ICaznarrQglZbN5MRglJ0LO75sqvScNkzMxMcg1j
onCMyRnki5Yz6zsvMMk8kPRgTuHyvNIPuHyab5clkaBo/ihiIGUiFBbmgh7SAx+h
f8xdBKuOsY7N3tp/3LTPw4qclcKEQkv10r+TLCBUFaMR7EpGU9eKxEW1YznjBTcB
PctlkZkrYF/ekN5iffpUxp0pmsE7Nxt2jJqxeTmCjOEikt3h9bnL53S4vlNMIPaX
pDvvr7NigRFzqXDUgiTqRpebofu/hnfgildlTEbmQi8DtN4yCa7YXpc5SMkjSdqX
0dYWQeLU/sTrekDT+lvQ0zM49ii9nMYCux2/HVu1I3qZPVjbZLtpVZAIV4nd9DdY
2wZwuLTK90QExqJcYb64FtSz7s7NURDj4LgDCIxg1nEMmVDQ+Nk6HBxpz/fQ2uWC
hiickYj1Thtzy2eJ+E3DL8R2w4MnyS/vLuoXuzUoyTjDH/00m+xG+d2Udcv9+Kpr
ssBuiRYNAZzPvBGv+85JrTGBbz272T3HPsVMGbKIXEYrzm7jIO96b+SRwszdBa4n
fT/KghvOfylpU2pv39FJiHcRFH2uObD+5TyLEB6OmVlZmjZaR2sNYd7j0DyBuZiL
VVx20kjqEDOq9WroOwk/PEla7L0wpa6+iJw0h8Y/38dxv8YT0Rb8HK9lCKmzxiL3
vGP3h3RKYIe8ry6FKStzycin3GmgZfQgWsIpO6o/v/NPLALjbGzCf7ZyEVwph+VE
Z9udKFKgwOMD1NvnCsVQkhR0VRaofMwoVLRVxjqwUnX7o2LKXmJG+vKgT5GXclQN
CSv3o9aAV0D4+j4dkeVotNOUoGlmSC9nCOe8xPIK4zkiC5IOuF5bhLyutVzajR7l
+Bc+83INNtpPtW8dXldo01/z4jcNfOM5j+YWe82OQ6ZRO9CQDtpE8rEH2f8Lp26+
ARKDLbZe7967sedO+S2pMTCEglsxrO7ByHfjHGa+FzOzaRLIkA4T4An/Bw2UW8kb
RRqsM3X/+h7y9QdefsQN6HxEK1xkEI+ffV/5T8cIy8Uged3Y9jWyIGmhWWCHoVUo
dXXbSqOWLxB5hvACh1Q/8mYs5GEylQrBXZZ8gnO+Y406V4L/FvV9M65VUTegucuh
qIKbe/HXnjtJ5IjSqcOcTXx9Rz3K6WuhB25lnzinm925Y08V7OG8DlHkPmxtwOuV
2Za/f3sPwamq575NNQf85Tuxzh6+IOhZMJrpFobZCMp2sG11pTT9BVRqOE0KnojF
fLvOE5QhbI7uIEvh6CMgFF7M1jtImSJXXwpcIjI7Q2S/W6aSz1MHiFPwRm+WvqTL
LJMCZML2vbQ56v1StFqZDhCiTFaMnHKDlCjqVmDbN0oWXD4zlQrkCcNoUh+wGb/B
K8+0gtCQOwixej807cqWwZASLHGnSk7mb+T80ZfWmPMqt4Bec35LThTo3y4PfptQ
-----END RSA PRIVATE KEY-----";

const PASSPHRASE: &str = "najidnadri";

#[derive(Debug, Deserialize, Serialize)]
pub enum ServerResponse {
    SaltReceived(String),
    SignupErr(SignupErr),
    LoginErr,
    ServerFailed,
    Ok,
    Err,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct SignupErr {
    pub email_taken: bool,
    pub phone_number_taken: bool,
    pub company_name_taken: bool,
}

#[derive(Debug)]
pub enum CheckCredsError {
    ServerError,
}

pub async fn decrypt_body(body: Buffer) -> String {
    println!("{:?}", body);
    let mut data = String::new();
    for bytes in body.bytes {
        let rsa = Rsa::private_key_from_pem_passphrase(PRIVATE_KEY.as_bytes(), PASSPHRASE.as_bytes()).unwrap();
        let mut buf: Vec<u8> = vec![0; rsa.size() as usize];
        let _ = rsa.private_decrypt(&bytes, &mut buf, Padding::PKCS1_OAEP).expect("err in decrypting data handler::decrypt_body");
        let result = String::from_utf8(buf).expect("err in parsing string from vec<u8> in server::handler::decrypt_body"); 
        let result = result.trim_matches(char::from(0)).to_string();
        data.push_str(&result);
    }
    let data = data.trim_matches(char::from(0)).to_string();
    data
}

pub async fn check_creds_exist_employee(conn: &mut PooledConnection<'_, ConnectionManager>,
    phone_number: &str,
    email: &str) -> Result<Option<SignupErr>, CheckCredsError> {
    
    let mut phonenumber_taken = false;
    let mut email_taken = false;
    
    //phone number
    //create query
    let query = query_phone_number_employee();
    let response = conn.query(&query, &[&phone_number.trim()]).await;
    if let Err(e) = &response {
        println!("error in executing query handler::chech_creds_exist_employee: {:?}", e);
        return Err(CheckCredsError::ServerError)
    }
    //check row exist
    let row_result = response.unwrap()
    .into_row()
    .await;
    if let Err(e) = &row_result {
        println!("error into_row() handler::check_creds_exist_employee: {:?}", e);
        return Err(CheckCredsError::ServerError)
    }
    if let Some(_) = &row_result.as_ref().unwrap() {
        println!("Phone number taken : query = {:?}", &query);
        phonenumber_taken = true;
    }

    //EMAIL
    //create query
    let query = query_email_employee();
    let response = conn.query(&query, &[&email.trim()]).await;
    if let Err(e) = &response {
        println!("error in executing query handler::chech_creds_exist_employee: {:?}", e);
        return Err(CheckCredsError::ServerError)
    }

    //check row exist
    let row_result = response.unwrap()
    .into_row()
    .await;
    if let Err(e) = &row_result {
        println!("error into_row() handler::check_creds_exist_employee: {:?}", e);
        return Err(CheckCredsError::ServerError)
    }
    if let Some(_) = &row_result.as_ref().unwrap() {
        println!("email taken : query = {:?}", &query);
        email_taken = true;
    }

    //conclude
    if !phonenumber_taken && !email_taken {
        return Ok(None)
    } else {
        let err = SignupErr {
            email_taken,
            phone_number_taken: phonenumber_taken,
            company_name_taken: false
        };
        return Ok(Some(err))
    }
}

pub async fn check_creds_exist_employer(conn: &mut PooledConnection<'_, ConnectionManager>,
    phone_number: &str,
    email: &str,
    companyname: &str) -> Result<Option<SignupErr>, CheckCredsError> {
    
    let mut phonenumber_taken = false;
    let mut email_taken = false;
    let mut companyname_taken = false;
    
    //phone number
    //create query
    let query = query_phone_number_employer();
    let response = conn.query(&query, &[&phone_number.trim()]).await;
    if let Err(e) = &response {
        println!("error in executing query handler::chech_creds_exist_employer: {:?}", e);
        return Err(CheckCredsError::ServerError)
    }
    //check row exist
    let row_result = response.unwrap()
    .into_row()
    .await;
    if let Err(e) = &row_result {
        println!("error into_row() handler::check_creds_exist_employer: {:?}", e);
        return Err(CheckCredsError::ServerError)
    }
    if let Some(_) = &row_result.as_ref().unwrap() {
        println!("Phone number taken : query = {:?}", &query);
        phonenumber_taken = true;
    }

    //EMAIL
    //create query
    let query = query_email_employer();
    let response = conn.query(&query, &[&email.trim()]).await;
    if let Err(e) = &response {
        println!("error in executing query handler::chech_creds_exist_employer: {:?}", e);
        return Err(CheckCredsError::ServerError)
    }

    //check row exist
    let row_result = response.unwrap()
    .into_row()
    .await;
    if let Err(e) = &row_result {
        println!("error into_row() handler::check_creds_exist_employer: {:?}", e);
        return Err(CheckCredsError::ServerError)
    }
    if let Some(_) = &row_result.as_ref().unwrap() {
        println!("email taken : query = {:?}", &query);
        email_taken = true;
    }

    //COMPANY NAME
    //create query
    let query = query_companyname();
    let response = conn.query(&query, &[&companyname.trim()]).await;
    if let Err(e) = &response {
        println!("error in executing query handler::check_creds_exist_employer: {:?}", e);
        return Err(CheckCredsError::ServerError)
    }

    //check row exist
    let row_result = response.unwrap()
    .into_row()
    .await;
    if let Err(e) = &row_result {
        println!("error into_row() handler::check_creds_exist_employer: {:?}", e);
        return Err(CheckCredsError::ServerError)
    }
    if let Some(_) = &row_result.as_ref().unwrap() {
        println!("company name taken : query = {:?}", &query);
        companyname_taken = true;
    }

    //conclude
    if !phonenumber_taken && !email_taken && !companyname_taken {
        return Ok(None)
    } else {
        let err = SignupErr {
            email_taken,
            phone_number_taken: phonenumber_taken,
            company_name_taken: companyname_taken,
        };
        return Ok(Some(err))
    }
}