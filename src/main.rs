#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release
use std::io::Write;
use eframe::egui;
use std::fs::{File, OpenOptions};
use std::io::Read;
fn main() -> Result<(), eframe::Error> {
    //let mut file = File::create("data/data.txt").expect("create failed");
    //file.write_all("Hello World".as_bytes()).expect("write failed");
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
    age: u32,
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
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
                            ui.label(format!("{}",self.age));
                            ui.set_min_height(200.0);
                        });
                        let _col = egui::collapsing_header::CollapsingHeader::new("Сотрудники")
                        .id_source(id)
                        .show(ui, |ui| {
                            ui.label("их пока нет");
                            //let mut but = ui.button("go");
                            if ui.button(format!("{}",id)).clicked() {
                                self.age += id as u32;
                                let mut file = OpenOptions::new().read(true).write(true).create(true).open("data/data.txt").unwrap();
                                //File::open("data/data.txt").expect("create failed");
                                let mut str1 = String::new();
                                file.read_to_string(&mut str1).expect("write failed");
                                file.write_all(format!("{}\n",self.age).as_bytes()).expect("write failed");
                                //writeln!(file,"{}",format!("{}",self.age));
                            };
                        });
                    });
                };
                // let name_label = ui.label("Your name: ");
                // ui.text_edit_singleline(&mut self.name)
                //     .labelled_by(name_label.id);
            });
        });
    }
}
