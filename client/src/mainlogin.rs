use egui::Color32;
use serde::{Deserialize, Serialize};

use crate::handler::{Event, Page, ClientRequest, send_request, ServerResponse};

#[derive(Debug, Deserialize, Serialize)]
pub struct EmployerLoginCreds {
    pub phonenumber: String,
    pub pass: String,
    pub pass_visible: bool
}

impl EmployerLoginCreds {
    pub fn new() -> Self {
        EmployerLoginCreds { phonenumber: String::new(), pass: String::new(), pass_visible: true }
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct EmployeeLoginCreds {
    pub phonenumber: String,
    pub pass: String,
    pub pass_visible: bool,
}

impl EmployeeLoginCreds {
    pub fn new() -> Self {
        EmployeeLoginCreds { phonenumber: String::new(), pass: String::new(), pass_visible: true }
    }
}

pub fn main_login(event: &mut Event, ctx: &egui::CtxRef) {
    egui::TopBottomPanel::top("header").default_height(100.).show(ctx, |ui| {
        ui.add_sized([100.0, 100.0],  egui::Label::new("KERJA\nSINI").underline().strong());
    });
    egui::CentralPanel::default().show(ctx, |ui| {
        ui.vertical_centered(|ui| {
            ui.add_space(50.);
            ui.allocate_ui([500.,500.].into(), |ui| {
                egui::Frame::none().fill(egui::Color32::LIGHT_BLUE).show(ui, |ui| {
                    ui.add_space(350.);
                    let foremployee_button = ui.add_sized([300., 30.], egui::Button::new("For Employee").fill(egui::Color32::BLUE).text_color(egui::Color32::WHITE));
                    ui.add_space(10.);
                    let foremployer_button = ui.add_sized([300., 30.], egui::Button::new("For Employer").fill(egui::Color32::BLUE).text_color(egui::Color32::WHITE));
                    ui.add_space(10.);
                    if foremployee_button.clicked() {
                        event.page = Page::EmployeeLogin;
                    }
                    if foremployer_button.clicked() {
                        event.page = Page::EmployerLogin;
                    }
                });
            });
        });
    });
}

pub fn employer_login(event: &mut Event, ctx: &egui::CtxRef) {
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
                    ui.colored_label(Color32::BLACK, "phone number");
                    ui.add_space(5.);
                    ui.add_sized([280., 20.], egui::TextEdit::singleline(&mut event.data.employer_login.phonenumber));
                    //pass
                    ui.add_space(10.);
                    ui.colored_label(Color32::BLACK, "Password");
                    ui.add_space(5.);
                    ui.add_sized([280., 20.], egui::TextEdit::singleline(&mut event.data.employer_login.pass).password(event.data.employer_login.pass_visible));
                    ui.add_space(1.);
                    ui.checkbox(&mut event.data.employer_login.pass_visible, "hide password");
                    //buttons
                    ui.add_space(20.);
                    ui.add_sized([300., 30.], egui::Button::new("Login").fill(egui::Color32::BLUE).text_color(egui::Color32::WHITE));
                    ui.add_space(10.);
                    let signup_button = ui.add_sized([300., 30.], egui::Button::new("Sign up as employer").fill(egui::Color32::BLUE).text_color(egui::Color32::WHITE));
                    let back_button = ui.add_sized([300., 30.], egui::Button::new("Back").fill(egui::Color32::GRAY).text_color(egui::Color32::WHITE));
                    ui.add_space(10.);
                    if signup_button.clicked() {
                        event.data.employer_login.pass.clear();
                        event.page = Page::EmployerSignup;
                    }
                    if back_button.clicked() {
                        event.data.employer_login.pass.clear();
                        event.page = Page::MainLogin;
                    }
                });
            });
        });
    });
}

pub fn employee_login(event: &mut Event, ctx: &egui::CtxRef) {
    egui::TopBottomPanel::top("header").default_height(100.).show(ctx, |ui| {
        ui.add_sized([100.0, 100.0],  egui::Label::new("KERJA\nSINI").underline().strong());
    });
    egui::CentralPanel::default().show(ctx, |ui| {
        ui.vertical_centered(|ui| {
            ui.add_space(50.);
            ui.allocate_ui([500.,500.].into(), |ui| {
                egui::Frame::none().fill(egui::Color32::LIGHT_BLUE).corner_radius(5.).show(ui, |ui| {
                    ui.add_space(30.);
                    //username
                    ui.colored_label(Color32::BLACK, "phone number");
                    ui.add_space(5.);
                    ui.add_sized([280., 20.], egui::TextEdit::singleline(&mut event.data.employer_login.phonenumber));
                    //pass
                    ui.add_space(10.);
                    ui.colored_label(Color32::BLACK, "Password");
                    ui.add_space(5.);
                    ui.add_sized([280., 20.], egui::TextEdit::singleline(&mut event.data.employer_login.pass).password(event.data.employer_login.pass_visible));
                    ui.add_space(1.);
                    ui.checkbox(&mut event.data.employer_login.pass_visible, "hide password");
                    //buttons
                    ui.add_space(20.);
                    let login_button = ui.add_sized([300., 30.], egui::Button::new("Login").fill(egui::Color32::BLUE).text_color(egui::Color32::WHITE));
                    ui.add_space(10.);
                    let signup_button = ui.add_sized([300., 30.], egui::Button::new("Sign up as employee").fill(egui::Color32::BLUE).text_color(egui::Color32::WHITE));
                    let back_button = ui.add_sized([300., 30.], egui::Button::new("Back").fill(egui::Color32::GRAY).text_color(egui::Color32::WHITE));
                    ui.add_space(10.);
                    if login_button.clicked() {
                        let client_request = ClientRequest::EmployeeLogin(event.data.employee_login.clone());
                        let response = send_request(client_request);
                        match response {
                            ServerResponse::Ok => {
                                event.page = Page::MainLogin;
                            }
                            _ => {
                                event.data.employee_login.phonenumber = "something is wrong".to_string();
                            }
                        }
                        event.data.employee_login.pass.clear();
                    }
                    if signup_button.clicked() {
                        event.data.employee_login.pass.clear();
                        event.page = Page::EmployeeSignup;
                    }
                    if back_button.clicked() {
                        event.data.employee_login.pass.clear();
                        event.page = Page::MainLogin;
                    }
                });
            });
        });
    });
}