use eframe::epi::App;

use crate::mainlogin;

pub struct Event {
    pub data: String,
    pub page: Page
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
            Page::EmployeeLogin => (),
            Page::EmployerLogin => (),
            Page::EmployeeSignup => (),
            Page::EmployerSignup=> (),
        }
    }

    fn name(&self) -> &str {
        "KerjaSini"
    }
}