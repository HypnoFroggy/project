#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use eframe::egui;
fn main() -> Result<(), eframe::Error> {
    env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).
    let options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(800.0, 600.0)),
        ..Default::default()
    };
    eframe::run_native(
        "Headmaster's monitoring app",
        options,
        Box::new(|_cc| Box::<MyApp>::default()),
    )
}

struct MyApp {
    name: String,
    age: u32,
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            name: "Arthur".to_owned(),
            age: 42,
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.set_height(300.0);
            ui.set_width(300.0);
            ui.horizontal(|ui| {
                let sections: [&str;3] = ["Отдел продаж", "Сборочный цех", "Программисты"];

                for (id, section) in sections.iter().enumerate() {
                    ui.vertical(|ui| {
                        ui.scope(|ui| {
                            ui.set_min_height(20.0);
                            ui.heading(*section);
                        });
                        ui.group(|ui| {
                            ui.label("Общая информация");
                            ui.set_min_height(200.0);
                        });
                        let mut col = egui::collapsing_header::CollapsingHeader::new("Сотрудники")
                        .id_source(id)
                        .show(ui, |ui| {
                            ui.label("их пока нет");

                            if ui.button("Click me").clicked() {
                                ui.label("их пока нет, но ты создал");
                            };
                        });
                    });
                }
                // let name_label = ui.label("Your name: ");
                // ui.text_edit_singleline(&mut self.name)
                //     .labelled_by(name_label.id);
            });
        });
    }
}
