use egui::Color32;
use serde::{Deserialize, Serialize};

use crate::handler::{Event, Page, ClientRequest, send_request, filter_response};

#[derive(Debug, Deserialize, Serialize)]
pub struct SignupErr {
    pub email_taken: bool,
    pub phone_number_taken: bool,
    pub company_name_taken: bool,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub enum PassStatus {
    TooShort,
    UniqueCharMissing,
    NumberMissing,
    UppercaseMissing,
    Good,
    Zero,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct EmployerSignupInfo {
    pub phonenumber: String,
    pub fullname: String,
    pub companyname: String,
    pub email: String,
    pub website: String,
    pub regnum: String,
    pub pass: String,
    pub salt: String,
    pub retype_pass: String,
    pub pass_visible: bool,
    pub postcode: String,
    pub pass_status: PassStatus,
    pub retype_pass_err: bool,
    pub email_taken: bool,
    pub phonenumber_taken: bool,
    pub companyname_taken: bool,
    pub signup_button: bool,
    pub email_err: bool,
    pub phonenumber_err: bool,
}

impl EmployerSignupInfo {
    pub fn new() -> Self {
        EmployerSignupInfo { email_err: false, phonenumber_err: false,  signup_button: false, companyname_taken: false, phonenumber_taken: false, email_taken: false, retype_pass_err: false, companyname: String::new(), fullname: String::new(), phonenumber: String::new(), website: String::new(), regnum: String::new(), pass: String::new(), salt: String::new(), email: String::new(), retype_pass: String::new(), pass_visible: true, postcode: String::new(), pass_status: PassStatus::Zero }
    }

    pub fn clear(&mut self) {
        self.phonenumber.clear();
        self.fullname.clear();
        self.companyname.clear();
        self.email.clear();
        self.website.clear();
        self.regnum.clear();
        self.pass.clear();
        self.salt.clear();
        self.retype_pass.clear();
        self.postcode.clear();
        self.pass_status = PassStatus::Zero;
        self.retype_pass_err = false;
        self.email_taken = false;
        self.phonenumber_taken = false;
        self.companyname_taken = false;
        self.signup_button = false;
        self.email_err = false;
        self.phonenumber_err = false;
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct EmployeeSignupInfo {
    pub phonenumber: String,
    pub fullname: String,
    pub username: String,
    pub email: String,
    pub age: String,
    pub pass: String,
    pub salt: String,
    pub retype_pass: String,
    pub pass_visible: bool,
    pub postcode: String,
    pub pass_status: PassStatus,
    pub retype_pass_err: bool,
    pub email_taken: bool,
    pub phonenumber_taken: bool,
    pub signup_button: bool,
    pub email_err: bool,
    pub phonenumber_err: bool,
}

impl EmployeeSignupInfo {
    pub fn new() -> Self {
        EmployeeSignupInfo { email_err: false, phonenumber_err: false, signup_button: false, retype_pass_err: false, email_taken: false, phonenumber_taken: false, pass_status: PassStatus::Zero, fullname: String::new(), username: String::new(), email: String::new(), age: String::new(), phonenumber: String::new(), pass: String::new(), salt: String::new(), retype_pass: String::new(), pass_visible: true, postcode: String::new() }
    }
    pub fn clear(&mut self) {
        self.phonenumber.clear();
        self.fullname.clear();
        self.username.clear();
        self.email.clear();
        self.age.clear();
        self.pass.clear();
        self.salt.clear();
        self.retype_pass.clear();
        self.postcode.clear();
        self.pass_status = PassStatus::Zero;
        self.retype_pass_err = false;
        self.email_taken = false;
        self.phonenumber_taken = false;
        self.signup_button = false;
        self.phonenumber_err = false;
        self.email_err = false;
    }
}

pub fn employer_signup(event: &mut Event, ctx: &egui::CtxRef) {
    //check signup button enable
    if event.data.employer_signup.pass_status == PassStatus::Good {
        if event.data.employer_signup.retype_pass_err == false &&
        event.data.employer_signup.email_err == false &&
        event.data.employer_signup.phonenumber_err == false &&
        !event.data.employer_signup.email.is_empty() &&
        !event.data.employer_signup.phonenumber.is_empty() &&
        !event.data.employer_signup.fullname.is_empty() &&
        !event.data.employer_signup.postcode.is_empty() &&
        !event.data.employer_signup.retype_pass.is_empty() &&
        !event.data.employer_signup.companyname.is_empty() &&
        !event.data.employer_signup.regnum.is_empty() {
            if event.data.employer_signup.pass.trim() == event.data.employer_signup.retype_pass.trim() {
                event.data.employer_signup.signup_button = true;
            } else {
                event.data.employer_signup.signup_button = false;
            }
        } else {
            event.data.employer_signup.signup_button = false;
        }
    } else {
        event.data.employer_signup.signup_button = false;
    }
    //header
    egui::TopBottomPanel::top("header").default_height(100.).show(ctx, |ui| {
        ui.add_sized([100.0, 100.0],  egui::Label::new("KERJA\nSINI").underline().strong());
    });

    //body
    egui::CentralPanel::default().show(ctx, |ui| {
        egui::ScrollArea::vertical().show(ui, |ui| {
            ui.vertical_centered(|ui| {
                ui.add_space(50.);
                ui.allocate_ui([500.,500.].into(), |ui| {
                    egui::Frame::none().fill(egui::Color32::LIGHT_BLUE).corner_radius(5.).show(ui, |ui| {
                        ui.add_space(30.);
                        //company name
                        ui.colored_label(Color32::BLACK, "company name");
                        if event.data.employer_signup.companyname_taken {
                            ui.colored_label(Color32::RED, "Company Name taken!");
                        }
                        ui.add_space(5.);
                        ui.add_sized([280., 20.], egui::TextEdit::singleline(&mut event.data.employer_signup.companyname));
                        //phone number
                        ui.add_space(10.);
                        ui.colored_label(Color32::BLACK, "Phone Number");
                        if event.data.employer_signup.phonenumber_taken {
                            ui.colored_label(Color32::RED, "Phone Number taken!");
                        }
                        if event.data.employer_signup.phonenumber_err {
                            ui.colored_label(Color32::RED, "Enter all digits only");
                        }
                        ui.add_space(5.);
                        let phonenumber_line = ui.add_sized([280., 20.], egui::TextEdit::singleline(&mut event.data.employer_signup.phonenumber));
                        //fullname
                        ui.add_space(10.);
                        ui.colored_label(Color32::BLACK, "Full Name");
                        ui.add_space(5.);
                        ui.add_sized([280., 20.], egui::TextEdit::singleline(&mut event.data.employer_signup.fullname));
                        //website
                        ui.add_space(10.);
                        ui.colored_label(Color32::BLACK, "Website");
                        ui.add_space(5.);
                        ui.add_sized([280., 20.], egui::TextEdit::singleline(&mut event.data.employer_signup.website));
                        //email
                        ui.add_space(10.);
                        ui.colored_label(Color32::BLACK, "Email");
                        if event.data.employer_signup.email_taken {
                            ui.colored_label(Color32::RED, "Email Taken!");
                        }
                        if event.data.employer_signup.email_err {
                            ui.colored_label(Color32::RED, "Wrong email format");
                        }
                        ui.add_space(5.);
                        let email_line = ui.add_sized([280., 20.], egui::TextEdit::singleline(&mut event.data.employer_signup.email));
                        //regnum
                        ui.add_space(10.);
                        ui.colored_label(Color32::BLACK, "Regnum");
                        ui.add_space(5.);
                        ui.add_sized([280., 20.], egui::TextEdit::singleline(&mut event.data.employer_signup.regnum));
                        //postcode
                        ui.add_space(10.);
                        ui.colored_label(Color32::BLACK, "Postcode");
                        ui.add_space(5.);
                        ui.add_sized([280., 20.], egui::TextEdit::singleline(&mut event.data.employer_signup.postcode));
                        //pass
                        ui.add_space(10.);
                        ui.colored_label(Color32::BLACK, "New Password");
                        match event.data.employer_signup.pass_status {
                            PassStatus::TooShort => {
                                ui.colored_label(Color32::RED, "The password is too short!");
                                ui.colored_label(Color32::LIGHT_RED, "Please include special char, number and uppercase letter in your password");
                            },
                            PassStatus::UniqueCharMissing => {
                                ui.colored_label(Color32::RED, "Missing unique char");
                                ui.colored_label(Color32::LIGHT_RED, "Please include special char, number and uppercase letter in your password");
                            },
                            PassStatus::NumberMissing => {
                                ui.colored_label(Color32::RED, "Number char missing");
                                ui.colored_label(Color32::LIGHT_RED, "Please include special char, number and uppercase letter in your password");
                            }
                            PassStatus::UppercaseMissing => {
                                ui.colored_label(Color32::RED, "uppercase char missing");
                                ui.colored_label(Color32::LIGHT_RED, "Please include special char, number and uppercase letter in your password");
                            },
                            PassStatus::Good => {
                                ui.colored_label(Color32::GREEN, "Now this is what I called a strong password");
                            }
                            PassStatus::Zero => (),
                        }
                        ui.add_space(5.);
                        let pass_line = ui.add_sized([280., 20.], egui::TextEdit::singleline(&mut event.data.employer_signup.pass).password(event.data.employer_signup.pass_visible));
                        ui.add_space(1.);
                        ui.checkbox(&mut event.data.employer_signup.pass_visible, "hide password");
                        //retype pass
                        ui.add_space(10.);
                        ui.colored_label(Color32::BLACK, "Retype New Password");
                        if event.data.employer_signup.retype_pass_err {
                            ui.colored_label(Color32::RED, "The passwords are not the same!!");
                        }
                        ui.add_space(5.);
                        let retype_pass = ui.add_sized([280., 20.], egui::TextEdit::singleline(&mut event.data.employer_signup.retype_pass));
                        //buttons
                        ui.add_space(20.);
                        //sign up button
                        let _signup_button = ui.add_enabled_ui(event.data.employer_signup.signup_button, |ui| {
                            let button = ui.add_sized([300., 30.], egui::Button::new("Sign up as employer").fill(egui::Color32::BLUE).text_color(egui::Color32::WHITE));
                            if button.clicked() {
                                event.data.employer_signup.retype_pass.clear();
                                //make client request
                                let client_request = ClientRequest::EmployerSignup(event.data.employer_signup.clone());
                                let response = send_request(client_request);
                                //filter response
                                filter_response(event, response);
                            }
                        });

                        let back_button = ui.add_sized([300., 30.], egui::Button::new("Back").fill(egui::Color32::GRAY).text_color(egui::Color32::WHITE));
                        ui.add_space(10.);

                        if back_button.clicked() {
                            event.data.employer_signup.pass.clear();
                            event.data.employer_signup.retype_pass.clear();
                            event.page = Page::EmployerLogin;
                        }
    
                        if retype_pass.lost_focus() {
                            if event.data.employer_signup.pass.trim() != event.data.employer_signup.retype_pass.trim() {
                                event.data.employer_signup.retype_pass_err = true;
                            } else {
                                event.data.employer_signup.retype_pass_err = false;
                            }
                        }

                        if phonenumber_line.lost_focus() {
                            if !event.data.employer_signup.phonenumber.clone().bytes().all(|c| c.is_ascii_digit()) {
                                event.data.employer_signup.phonenumber_err = true;
                            } else {
                                event.data.employer_signup.phonenumber_err = false;
                            }
                        }

                        if email_line.lost_focus() {
                            if !event.data.employer_signup.email.clone().contains('@') {
                                event.data.employer_signup.email_err = true;
                            } else {
                                event.data.employer_signup.email_err = false;
                            }
                        }
    
                        if pass_line.lost_focus() {
                            let pass_status = check_password_secure(&event.data.employer_signup.pass);
                            event.data.employer_signup.pass_status = pass_status;
                        }
                    });
                });
            });
        });
    });
}


pub fn employee_signup(event: &mut Event, ctx: &egui::CtxRef) {
    //check signup button enable
    if event.data.employee_signup.pass_status == PassStatus::Good {
        if event.data.employee_signup.retype_pass_err == false &&
        event.data.employee_signup.email_err == false &&
        event.data.employee_signup.phonenumber_err == false &&
        !event.data.employee_signup.email.is_empty() &&
        !event.data.employee_signup.phonenumber.is_empty() &&
        !event.data.employee_signup.fullname.is_empty() &&
        !event.data.employee_signup.postcode.is_empty() &&
        !event.data.employee_signup.retype_pass.is_empty() &&
        !event.data.employee_signup.username.is_empty() &&
        !event.data.employee_signup.age.is_empty() {
            if event.data.employee_signup.pass.trim() == event.data.employee_signup.retype_pass.trim() {
                event.data.employee_signup.signup_button = true;
            } else {
                event.data.employee_signup.signup_button = false;
            }
        } else {
            event.data.employee_signup.signup_button = false;
        }
    } else {
        event.data.employee_signup.signup_button = false;
    }
    //header
    egui::TopBottomPanel::top("header").default_height(100.).show(ctx, |ui| {
        ui.add_sized([100.0, 100.0],  egui::Label::new("KERJA\nSINI").underline().strong());
    });
    //body
    egui::CentralPanel::default().show(ctx, |ui| {
        egui::ScrollArea::vertical().show(ui, |ui| {
            ui.vertical_centered(|ui| {
                ui.add_space(50.);
                ui.allocate_ui([500.,500.].into(), |ui| {
                    egui::Frame::none().fill(egui::Color32::LIGHT_BLUE).corner_radius(5.).show(ui, |ui| {
                        ui.add_space(30.);
                        //full name
                        ui.colored_label(Color32::BLACK, "Full name");
                        ui.add_space(5.);
                        ui.add_sized([280., 20.], egui::TextEdit::singleline(&mut event.data.employee_signup.fullname));
                        //username
                        ui.add_space(10.);
                        ui.colored_label(Color32::BLACK, "Username");
                        ui.add_space(5.);
                        ui.add_sized([280., 20.], egui::TextEdit::singleline(&mut event.data.employee_signup.username));
                        //phone number
                        ui.add_space(10.);
                        ui.colored_label(Color32::BLACK, "Phone Number");
                        if event.data.employee_signup.phonenumber_taken {
                            ui.colored_label(Color32::RED, "Phone Number taken!");
                        }
                        if event.data.employee_signup.phonenumber_err {
                            ui.colored_label(Color32::RED, "Enter all digits only");
                        }
                        ui.add_space(5.);
                        let phonenumber_line = ui.add_sized([280., 20.], egui::TextEdit::singleline(&mut event.data.employee_signup.phonenumber));
                        //email
                        ui.add_space(10.);
                        ui.colored_label(Color32::BLACK, "Email");
                        if event.data.employee_signup.email_taken {
                            ui.colored_label(Color32::RED, "Email Taken!");
                        }
                        if event.data.employee_signup.email_err {
                            ui.colored_label(Color32::RED, "Wrong email format");
                        }
                        ui.add_space(5.);
                        let email_line = ui.add_sized([280., 20.], egui::TextEdit::singleline(&mut event.data.employee_signup.email));
                        //age
                        ui.add_space(10.);
                        ui.colored_label(Color32::BLACK, "Age");
                        ui.add_space(5.);
                        ui.add_sized([280., 20.], egui::TextEdit::singleline(&mut event.data.employee_signup.age));
                        //postcode
                        ui.add_space(10.);
                        ui.colored_label(Color32::BLACK, "Postcode");
                        ui.add_space(5.);
                        ui.add_sized([280., 20.], egui::TextEdit::singleline(&mut event.data.employee_signup.postcode));
                        //pass
                        ui.add_space(10.);
                        ui.colored_label(Color32::BLACK, "New Password");
                        match event.data.employee_signup.pass_status {
                            PassStatus::TooShort => {
                                ui.colored_label(Color32::RED, "The password is too short!");
                                ui.colored_label(Color32::LIGHT_RED, "Please include special char, number and uppercase letter in your password");
                            },
                            PassStatus::UniqueCharMissing => {
                                ui.colored_label(Color32::RED, "Missing unique char");
                                ui.colored_label(Color32::LIGHT_RED, "Please include special char, number and uppercase letter in your password");
                            },
                            PassStatus::NumberMissing => {
                                ui.colored_label(Color32::RED, "Number char missing");
                                ui.colored_label(Color32::LIGHT_RED, "Please include special char, number and uppercase letter in your password");
                            }
                            PassStatus::UppercaseMissing => {
                                ui.colored_label(Color32::RED, "uppercase char missing");
                                ui.colored_label(Color32::LIGHT_RED, "Please include special char, number and uppercase letter in your password");
                            },
                            PassStatus::Good => {
                                ui.colored_label(Color32::GREEN, "Now this is what I called a strong password");
                            }
                            PassStatus::Zero => (),
                        }
                        ui.add_space(5.);
                        let pass_line = ui.add_sized([280., 20.], egui::TextEdit::singleline(&mut event.data.employee_signup.pass).password(event.data.employee_signup.pass_visible));
                        ui.add_space(1.);
                        ui.checkbox(&mut event.data.employee_signup.pass_visible, "hide password");
                        //retype pass
                        ui.add_space(10.);
                        ui.colored_label(Color32::BLACK, "Retype New Password");
                        if event.data.employee_signup.retype_pass_err {
                            ui.colored_label(Color32::RED, "The passwords are not the same!!");
                        }
                        ui.add_space(5.);
                        let retype_pass = ui.add_sized([280., 20.], egui::TextEdit::singleline(&mut event.data.employee_signup.retype_pass));
                        //buttons
                        ui.add_space(20.);
                        let _signup_button = ui.add_enabled_ui(event.data.employee_signup.signup_button, |ui| {
                            let button = ui.add_sized([300., 30.], egui::Button::new("Sign up as employee").fill(egui::Color32::BLUE).text_color(egui::Color32::WHITE));
                            if button.clicked() {
                                event.data.employee_signup.retype_pass.clear();
                                //make client request
                                let client_request = ClientRequest::EmployeeSignup(event.data.employee_signup.clone());
                                let response = send_request(client_request);
                                filter_response(event, response);
                            }
                        });
                        let back_button = ui.add_sized([300., 30.], egui::Button::new("Back").fill(egui::Color32::GRAY).text_color(egui::Color32::WHITE));
                        ui.add_space(10.);
                        if back_button.clicked() {
                            event.data.employee_signup.pass.clear();
                            event.data.employee_signup.retype_pass.clear();
                            event.page = Page::EmployeeLogin;
                        }
    
                        if retype_pass.lost_focus() {
                            if event.data.employee_signup.pass.trim() != event.data.employee_signup.retype_pass.trim() {
                                event.data.employee_signup.retype_pass_err = true;
                            } else {
                                event.data.employee_signup.retype_pass_err = false;
                            }
                        }

                        if phonenumber_line.lost_focus() {
                            if !event.data.employee_signup.phonenumber.clone().bytes().all(|c| c.is_ascii_digit()) {
                                event.data.employee_signup.phonenumber_err = true;
                            } else {
                                event.data.employee_signup.phonenumber_err = false;
                            }
                        }

                        if email_line.lost_focus() {
                            if !event.data.employee_signup.email.clone().contains('@') {
                                event.data.employee_signup.email_err = true;
                            } else {
                                event.data.employee_signup.email_err = false;
                            }
                        }
    
                        if pass_line.lost_focus() {
                            let pass_status = check_password_secure(&event.data.employee_signup.pass);
                            event.data.employee_signup.pass_status = pass_status;
                        }
                    });
                });
            });
        });
        
    });
}

fn check_password_secure(pass: &str) -> PassStatus {
    let mut number = false;
    let mut other_char = false;
    let mut uppercase = false;
    let mut amount = 0;

    for i in pass.chars() {
        if i.is_ascii_uppercase() {
            uppercase = true
        }

        match i {
            '1' => number = true,
            '2' => number = true,
            '3' => number = true,
            '4' => number = true,
            '5' => number = true,
            '6' => number = true,
            '7' => number = true,
            '8' => number = true,
            '9' => number = true,
            '_' => other_char = true,
            '@' => other_char = true,
            '!' => other_char = true,
            '#' => other_char = true,
            '%' => other_char = true,
            '&' => other_char = true,
            '*' => other_char = true,
            '^' => other_char = true,
            _ => {}
        }

        amount += 1;
    }

    if number == false {
        return PassStatus::NumberMissing;
    } else if other_char == false {
        return PassStatus::UniqueCharMissing;
    } else if uppercase == false {
        return PassStatus::UppercaseMissing;
    } else if amount < 6 {
        return PassStatus::TooShort;
    } else {
        return PassStatus::Good;
    }
    
}

