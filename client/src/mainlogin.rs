use crate::handler::{Event, Page};

pub struct EmployerLoginCreds {
    pub name: String,
    pub pass: String,
}

impl EmployerLoginCreds {
    pub fn new() -> Self {
        EmployerLoginCreds { name: String::new(), pass: String::new() }
    }
}

pub struct EmployeeLoginCreds {
    pub name: String,
    pub pass: String,
}

impl EmployeeLoginCreds {
    pub fn new() -> Self {
        EmployeeLoginCreds { name: String::new(), pass: String::new() }
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
                    ui.label("Company Name");
                    ui.add_space(10.);
                    ui.add_sized([280., 20.], egui::TextEdit::singleline(&mut event.data.employer_login.name));
                    ui.add_space(20.);
                    ui.label("Password");
                    ui.add_space(10.);
                    ui.add_sized([280., 20.], egui::TextEdit::singleline(&mut event.data.employer_login.pass).password(true));
                    ui.add_space(20.);
                    ui.add_sized([300., 30.], egui::Button::new("Login").fill(egui::Color32::BLUE).text_color(egui::Color32::WHITE));
                    ui.add_space(10.);
                    ui.add_sized([300., 30.], egui::Button::new("Sign up as employer").fill(egui::Color32::BLUE).text_color(egui::Color32::WHITE));
                    ui.add_space(10.);
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
                    ui.label("Username");
                    ui.add_space(10.);
                    ui.add_sized([280., 20.], egui::TextEdit::singleline(&mut event.data.employer_login.name));
                    ui.add_space(20.);
                    ui.label("Password");
                    ui.add_space(10.);
                    ui.add_sized([280., 20.], egui::TextEdit::singleline(&mut event.data.employer_login.pass).password(true));
                    ui.add_space(20.);
                    ui.add_sized([300., 30.], egui::Button::new("Login").fill(egui::Color32::BLUE).text_color(egui::Color32::WHITE));
                    ui.add_space(10.);
                    ui.add_sized([300., 30.], egui::Button::new("Sign up as employee").fill(egui::Color32::BLUE).text_color(egui::Color32::WHITE));
                    ui.add_space(10.);
                });
            });
        });
    });
}