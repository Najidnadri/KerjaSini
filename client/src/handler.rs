use eframe::epi::App;

use crate::{mainlogin::{self, EmployeeLoginCreds, EmployerLoginCreds}, main};

pub struct Event {
    pub data: Data,
    pub page: Page
}

pub struct Data {
    pub employee_login: EmployeeLoginCreds,
    pub employer_login: EmployerLoginCreds,
}

impl Data {
    pub fn new() -> Self {
        Data { employee_login: EmployeeLoginCreds::new(), employer_login: EmployerLoginCreds::new() }
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
            Page::EmployeeSignup => (),
            Page::EmployerSignup=> (),
        }
    }

    fn name(&self) -> &str {
        "KerjaSini"
    }
}