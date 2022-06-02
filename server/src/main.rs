struct LoginDetails {
    creds: Creds,
    pass: String,
}

enum Creds {
    Email,
    Name,
    PhoneNumber,
}

struct SignupDetails {
    name: String,
    email: String,
    phone_number: String,
    password: String,
    postcode: String,
}

impl SignupDetails {
    fn new() -> Self {
        SignupDetails { name: String::new(), email: String::new(), phone_number: String::new(), password: String::new(), postcode: String::new() }
    }
}

fn main() {
    //set up demo data
    let signup_demo = SignupDetails{
        name: "Najidnadri".to_string(),
        email: "muhd.najid.nadri@gmail.com".to_string(),
        phone_number: "0193187167".to_string(),
        password: "12345678".to_string(),
        postcode: "143143".to_string(),
    };

    
}
