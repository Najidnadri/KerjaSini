

use crate::handler::Event;



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
                    ui.add_sized([300., 30.], egui::Button::new("Employee Login").fill(egui::Color32::BLUE).text_color(egui::Color32::WHITE));
                    ui.add_space(10.);
                    ui.add_sized([300., 30.], egui::Button::new("Employer Login").fill(egui::Color32::BLUE).text_color(egui::Color32::WHITE));
                    ui.add_space(10.);
                });
            });
        });
    });
}