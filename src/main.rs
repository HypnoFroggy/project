#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release
//use std::io::Write;
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
struct sector {
    info: String,
    empl: String,
}
struct MyApp {
    dropcheck: bool,
    sells: sector,
    assembly: sector,
    program: sector,
}

impl Default for MyApp {
    fn default() -> Self {
        let mut paths: [&str; 6] = 
        ["data/sells_info.txt","data/sells_employees.txt",
        "data/assembly_info.txt","data/assembly_employees.txt",
        "data/program_info.txt","data/program_employees.txt",];
        let mut arr: [String; 6] = ["".to_string(),"".to_string(),"".to_string(),"".to_string(),"".to_string(),"".to_string()];
        for (num,path) in paths.iter().enumerate() {
            File::open(path).expect(path).read_to_string(&mut arr[num]);
        }
        Self {
            dropcheck: true,
            sells: sector  {
                info:  arr[0].clone(),
                empl:  arr[1].clone(),
            },
            assembly: sector   {
                info:  arr[2].clone(),
                empl:  arr[3].clone(),
            },
            program: sector   {
                info:  arr[4].clone(),
                empl:  arr[5].clone(),
            },
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
                            ui.set_height(10.0);
                            ui.set_width(200.0);
                            ui.heading(*section);
                        });
                        ui.group(|ui| {
                            ui.set_height(100.0);
                            ui.set_width(200.0);
                            ui.label("Общая информация");
                            if id == 0 {
                                ui.label(format!("{}",self.sells.info));
                            }
                            else if id == 1 {
                                ui.label(format!("{}",self.assembly.info));
                            }
                            else {
                                ui.label(format!("{}",self.program.info));
                            }
                        });
                        ui.group(|ui| {
                            ui.set_height(100.0);
                            ui.set_width(200.0);
                            ui.horizontal(|ui| {
                                ui.label("Задачи:");
                                //ui.label();
                            });
                        });
                        let _col = egui::collapsing_header::CollapsingHeader::new("Сотрудники")
                        .id_source(id)
                        .show(ui, |ui| {
                            ui.label("их пока нет");
                        });
                    });
                };
            });
            egui::Grid::new("some_unique_id").show(ui, |ui| {
                if ui.button("dropcheck").clicked() {
                    let x = self.dropcheck;
                    self.dropcheck = !x;
                };
                ui.radio_value(&mut self.dropcheck, true, "xt");
            });
            
        });
    }
}
