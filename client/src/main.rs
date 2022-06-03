mod handler;
mod mainlogin;
mod signup;

use egui::{self, Vec2};
use handler::{Event, Page, Data};
use tokio;
use eframe::{self, NativeOptions, run_native};

#[tokio::main]
async fn main() {
    let app = Event {
        data: Data::new(),
        page: Page::MainLogin,
    };

    let mut native_option = NativeOptions::default();
    native_option.initial_window_size = std::option::Option::Some(Vec2 { x: 1000., y: 800. });
    native_option.resizable = false;

    run_native(Box::new(app), native_option);
}
