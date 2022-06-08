mod handler;
mod mainlogin;
mod signup;

use egui::{self, Vec2};
use handler::{Event, Page, Data};
use tokio;
use eframe::{self, NativeOptions, run_native};

const PUBLIC_KEY: &str = "-----BEGIN PUBLIC KEY-----
MIGfMA0GCSqGSIb3DQEBAQUAA4GNADCBiQKBgQDCyIwoKqA6zofnddt4BK/plQBe
xcBWDlGnFIseXjenYOzCfzzaVabPECzFJg89IFT2zm5UnUZ3guQDvJSR5E6xYJfK
1gbPkgyU/Yk9Y5y1BEL5mfLTYeszzC9EbUa0F29NEYVWXi56xDP/3IHUBaKpDLVu
6495sjx+2h7USfOFvwIDAQAB
-----END PUBLIC KEY-----";

fn main() {
    let app = Event {
        data: Data::new(),
        page: Page::MainLogin,
    };

    let mut native_option = NativeOptions::default();
    native_option.initial_window_size = std::option::Option::Some(Vec2 { x: 1000., y: 800. });
    native_option.resizable = false;

    run_native(Box::new(app), native_option);
}
