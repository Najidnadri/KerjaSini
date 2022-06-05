use openssl::rsa::{Rsa, Padding};


const PRIVATE_KEY: &str = "-----BEGIN RSA PRIVATE KEY-----
Proc-Type: 4,ENCRYPTED
DEK-Info: AES-128-CBC,423FA8AA1F0D49966C646FB532C8BE9A

ait5aczSvzWWoBDraErDHKm4ps4jElkQebPxY/jSqNgOOfvxj9x0ewO8pp7z0cho
aTOg8KMzk08FfGmRZ/KvT9yQut21W37JIgBEMKTdDoFXCic1nZIGSdM/7EMTem/z
NBLk0Wy56EN22eUm7zw1avy3E1Rl8EbMQSrjBws8bDCpDG5j/TrbLhdGLKCoKLdN
cwF7q3YAt9axW+TXLx18hBEmJ09O/jnP4KsybYFLr8W3H0knZTDob04lVY08DNw0
hozW7cc8iRKuUQ6cfBCx/nl2rFdC84Ee4b1UPu8hAJJ0GBQ4WbyHlI3+bZQYNrrS
YAh6gXqvdp8Eyw9wdM+/bRAUFR5ewl+pTmBvM9Y7CFD8G4d1cVuXiZLYsT70eUPD
EsneDq/RGBLgu57kn69OHitAiohMxqq2iqsXwNNgYk5S6+zSVmRAXVYhXH55L88A
GZ4s2SsH1tWJPvxpqBlQ3E8d9pta7U9d+TZQ2DYsrXc2VMGZkP54R9OOOcceobsN
N49MXC7Iw45ogRUWIXaBOLyJoUJtcvi55PA/oTq7etPqFKUZT4JlEaO00MYoonEn
e/Cu8dxV/IYzMOEOhu+/WkJQxlyYQLTIKAiYxmBv5UR8INx2QXT14K1FZg/5LawW
4X0h8co/qDkCBGpKwLGVntahsP9pbMkGAlXh6ZBZNhA7Fl0GSi/0iEiR02kO1hvF
WSSPFTOkL08DP0lQ/oMm6zDV2vmkABcrQxBibS7n5lWfhvsNcQd97T3cjCEv67Vu
HxqPdksIJKZr0udu+TFAA2ZguvqHd2bpOl/mothfzqGDYchlDi7H3zXwlH5wKvxD
-----END RSA PRIVATE KEY-----";

const PASSPHRASE: &str = "najidnadri";

pub async fn decrypt_body(body: String) -> String {
    let rsa = Rsa::private_key_from_pem_passphrase(PRIVATE_KEY.as_bytes(), PASSPHRASE.as_bytes()).unwrap();
    let mut buf: Vec<u8> = vec![0; rsa.size() as usize];
    let _ = rsa.private_decrypt(&body.as_bytes(), &mut buf, Padding::PKCS1).unwrap();
    String::from_utf8(buf).expect("err in parsing string from vec<u8> in server::handler::decrypt_body")
}