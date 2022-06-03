use egui::Color32;

use crate::handler::{Event, Page};

pub struct EmployerSignupInfo {
    pub companyname: String,
    pub phonenumber: String,
    pub website: String,
    pub regnum: String,
    pub pass: String,
    pub retype_pass: String,
    pub pass_visible: bool,
    pub email: String,
}

impl EmployerSignupInfo {
    pub fn new() -> Self {
        EmployerSignupInfo { companyname: String::new(), phonenumber: String::new(), website: String::new(), regnum: String::new(), pass: String::new(), email: String::new(), retype_pass: String::new(), pass_visible: true }
    }
}

pub struct EmployeeSignupInfo {
    pub fullname: String,
    pub username: String,
    pub email: String,
    pub age: String,
    pub phonenumber: String,
    pub pass: String,
    pub retype_pass: String,
    pub pass_visible: bool,
}

impl EmployeeSignupInfo {
    pub fn new() -> Self {
        EmployeeSignupInfo { fullname: String::new(), username: String::new(), email: String::new(), age: String::new(), phonenumber: String::new(), pass: String::new(), retype_pass: String::new(), pass_visible: true }
    }
}

pub fn employer_signup(event: &mut Event, ctx: &egui::CtxRef) {
    egui::TopBottomPanel::top("header").default_height(100.).show(ctx, |ui| {
        ui.add_sized([100.0, 100.0],  egui::Label::new("KERJA\nSINI").underline().strong());
    });
    egui::CentralPanel::default().show(ctx, |ui| {
        ui.vertical_centered(|ui| {
            ui.add_space(50.);
            ui.allocate_ui([500.,500.].into(), |ui| {
                egui::Frame::none().fill(egui::Color32::LIGHT_BLUE).corner_radius(5.).show(ui, |ui| {
                    ui.add_space(30.);
                    //company name
                    ui.colored_label(Color32::BLACK, "company name");
                    ui.add_space(5.);
                    ui.add_sized([280., 20.], egui::TextEdit::singleline(&mut event.data.employer_signup.companyname));
                    //phone number
                    ui.add_space(10.);
                    ui.colored_label(Color32::BLACK, "Phone Number");
                    ui.add_space(5.);
                    ui.add_sized([280., 20.], egui::TextEdit::singleline(&mut event.data.employer_signup.phonenumber));
                    //website
                    ui.add_space(10.);
                    ui.colored_label(Color32::BLACK, "Website");
                    ui.add_space(5.);
                    ui.add_sized([280., 20.], egui::TextEdit::singleline(&mut event.data.employer_signup.website));
                    //email
                    ui.add_space(10.);
                    ui.colored_label(Color32::BLACK, "Email");
                    ui.add_space(5.);
                    ui.add_sized([280., 20.], egui::TextEdit::singleline(&mut event.data.employer_signup.email));
                    //regnum
                    ui.add_space(10.);
                    ui.colored_label(Color32::BLACK, "Regnum");
                    ui.add_space(5.);
                    ui.add_sized([280., 20.], egui::TextEdit::singleline(&mut event.data.employer_signup.regnum));
                    //pass
                    ui.add_space(10.);
                    ui.colored_label(Color32::BLACK, "New Password");
                    ui.add_space(5.);
                    ui.add_sized([280., 20.], egui::TextEdit::singleline(&mut event.data.employer_signup.pass).password(event.data.employer_signup.pass_visible));
                    ui.add_space(1.);
                    ui.checkbox(&mut event.data.employer_signup.pass_visible, "hide password");
                    //retype pass
                    ui.add_space(10.);
                    ui.colored_label(Color32::BLACK, "Retype New Password");
                    ui.add_space(5.);
                    ui.add_sized([280., 20.], egui::TextEdit::singleline(&mut event.data.employer_signup.retype_pass));
                    //buttons
                    ui.add_space(20.);
                    let signup_button = ui.add_sized([300., 30.], egui::Button::new("Sign up as employer").fill(egui::Color32::BLUE).text_color(egui::Color32::WHITE));
                    let back_button = ui.add_sized([300., 30.], egui::Button::new("Back").fill(egui::Color32::GRAY).text_color(egui::Color32::WHITE));
                    ui.add_space(10.);
                    if signup_button.clicked() {
                        event.page = Page::EmployerSignup;
                    }
                    if back_button.clicked() {
                        event.data.employer_signup.pass.clear();
                        event.data.employer_signup.retype_pass.clear();
                        event.page = Page::EmployerLogin;
                    }
                });
            });
        });
    });
}


pub fn employee_signup(event: &mut Event, ctx: &egui::CtxRef) {
    egui::TopBottomPanel::top("header").default_height(100.).show(ctx, |ui| {
        ui.add_sized([100.0, 100.0],  egui::Label::new("KERJA\nSINI").underline().strong());
    });
    egui::CentralPanel::default().show(ctx, |ui| {
        ui.vertical_centered(|ui| {
            ui.add_space(50.);
            ui.allocate_ui([500.,500.].into(), |ui| {
                egui::Frame::none().fill(egui::Color32::LIGHT_BLUE).corner_radius(5.).show(ui, |ui| {
                    ui.add_space(30.);
                    //company name
                    ui.colored_label(Color32::BLACK, "Full name");
                    ui.add_space(5.);
                    ui.add_sized([280., 20.], egui::TextEdit::singleline(&mut event.data.employee_signup.fullname));
                    //phone number
                    ui.add_space(10.);
                    ui.colored_label(Color32::BLACK, "Username");
                    ui.add_space(5.);
                    ui.add_sized([280., 20.], egui::TextEdit::singleline(&mut event.data.employee_signup.username));
                    //website
                    ui.add_space(10.);
                    ui.colored_label(Color32::BLACK, "Phone Number");
                    ui.add_space(5.);
                    ui.add_sized([280., 20.], egui::TextEdit::singleline(&mut event.data.employee_signup.phonenumber));
                    //email
                    ui.add_space(10.);
                    ui.colored_label(Color32::BLACK, "Email");
                    ui.add_space(5.);
                    ui.add_sized([280., 20.], egui::TextEdit::singleline(&mut event.data.employee_signup.email));
                    //regnum
                    ui.add_space(10.);
                    ui.colored_label(Color32::BLACK, "Age");
                    ui.add_space(5.);
                    ui.add_sized([280., 20.], egui::TextEdit::singleline(&mut event.data.employee_signup.age));
                    //pass
                    ui.add_space(10.);
                    ui.colored_label(Color32::BLACK, "New Password");
                    ui.add_space(5.);
                    ui.add_sized([280., 20.], egui::TextEdit::singleline(&mut event.data.employee_signup.pass).password(event.data.employee_signup.pass_visible));
                    ui.add_space(1.);
                    ui.checkbox(&mut event.data.employee_signup.pass_visible, "hide password");
                    //retype pass
                    ui.add_space(10.);
                    ui.colored_label(Color32::BLACK, "Retype New Password");
                    ui.add_space(5.);
                    ui.add_sized([280., 20.], egui::TextEdit::singleline(&mut event.data.employee_signup.retype_pass));
                    //buttons
                    ui.add_space(20.);
                    let signup_button = ui.add_sized([300., 30.], egui::Button::new("Sign up as employer").fill(egui::Color32::BLUE).text_color(egui::Color32::WHITE));
                    let back_button = ui.add_sized([300., 30.], egui::Button::new("Back").fill(egui::Color32::GRAY).text_color(egui::Color32::WHITE));
                    ui.add_space(10.);
                    if signup_button.clicked() {
                        event.page = Page::EmployerSignup;
                    }
                    if back_button.clicked() {
                        event.data.employer_signup.pass.clear();
                        event.data.employer_signup.retype_pass.clear();
                        event.page = Page::EmployerLogin;
                    }
                });
            });
        });
    });
}