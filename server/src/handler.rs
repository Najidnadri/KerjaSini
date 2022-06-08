use openssl::rsa::{Rsa, Padding};
use serde::{Deserialize, Serialize};

use crate::Buffer;


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

pub async fn decrypt_body(body: Buffer) -> String {
    let mut data = String::new();
    for bytes in body.bytes {
        let rsa = Rsa::private_key_from_pem_passphrase(PRIVATE_KEY.as_bytes(), PASSPHRASE.as_bytes()).unwrap();
        let mut buf: Vec<u8> = vec![0; rsa.size() as usize];
        let _ = rsa.private_decrypt(&bytes, &mut buf, Padding::PKCS1).expect("err in decrypting data handler::decrypt_body");
        let result = String::from_utf8(buf).expect("err in parsing string from vec<u8> in server::handler::decrypt_body"); 
        let result = result.trim_matches(char::from(0)).to_string();
        data.push_str(&result);
    }
    let data = data.trim_matches(char::from(0)).to_string();
    data
}
