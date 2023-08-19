#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use eframe::egui;
use std::fmt::write;
use std::fs::File;
use std::io::{self, BufRead, Read, Write};
use std::path::Path;
fn main() -> Result<(), eframe::Error> {
    env_logger::init();

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

fn read_lines<P>(filename: P) -> io::Lines<io::BufReader<File>>
where P: AsRef<Path>, {
    let file = File::open(filename).unwrap();
    io::BufReader::new(file).lines()
}

fn delete_line(counter: i32, stringg: String) -> String{
    let mut quote = "".to_string();
    File::open(&stringg).expect(&stringg).read_to_string(&mut quote);
    let bytes = quote.as_bytes();
    let mut lines: Vec<usize> = vec![0];
    for (i, &item) in bytes.iter().enumerate() {
        if item == b'\n' {
            lines.push(i);
        }
    }
    lines.push(quote.len());

    println!("счетчик {}",counter);
    //println!("хранимое {}",lines[counter as usize]);
    //println!("хранимое {}",lines[(counter+1) as usize]);

    //println!("{}\n",&quote[0..lines[lines.len()-1]]);
    if lines.len() == 2{
        "".to_string()
    }
    else if counter == 0 {
        print!("{}",quote[lines[1]..lines[lines.len()-1]].to_string());
        quote[lines[1]+1..lines[lines.len()-1]].to_string()
    }
    else {
        quote[0..lines[counter as usize]].to_string() + &quote[lines[(counter+1) as usize]..lines[lines.len()-1]]
    }
    
    //quote[lines[counter as usize]..lines[(counter+1) as usize]].to_string()
    

    //((stringg[0..lines[(counter-1) as usize]]).to_string()
    //+&(stringg[lines[(counter) as usize]..lines[lines.len()-1]]).to_string()).to_string()
}
struct Sector {
    info: String,
    empl: String,
    task: String
}
struct MyApp {
    sells: Sector,
    assembly: Sector,
    program: Sector,
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            sells: Sector  {
                info:  "data/sells_info.txt".to_string(),
                empl:  "data/sells_employees.txt".to_string(),
                task:  "data/sells_tasks.txt".to_string()
            },
            assembly: Sector   {
                info:  "data/assembly_info.txt".to_string(),
                empl:  "data/assembly_employees.txt".to_string(),
                task:  "data/assembly_tasks.txt".to_string()
            },
            program: Sector   {
                info:  "data/program_info.txt".to_string(),
                empl:  "data/program_employees.txt".to_string(),
                task:  "data/program_tasks.txt".to_string()
            },
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
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
                            ui.label("Общая информация:");
                            if id == 0 {
                                let mut lab = "".to_string();
                                File::open(&self.sells.info).expect(&self.sells.info).read_to_string(&mut lab);
                                ui.label(format!("{}",lab));
                            }
                            else if id == 1 {
                                let mut lab = "".to_string();
                                File::open(&self.assembly.info).expect(&self.assembly.info).read_to_string(&mut lab);
                                ui.label(format!("{}",lab));
                            }
                            else {
                                let mut lab = "".to_string();
                                File::open(&self.program.info).expect(&self.program.info).read_to_string(&mut lab);
                                ui.label(format!("{}",lab));
                            }
                        });
                        ui.group(|ui| {
                            ui.vertical(|ui| {
                                ui.label("Задачи:");
                                if id == 0 {
                                    let mut counter = 0;
                                    for lines in read_lines(self.sells.task.clone()){
                                        for line in lines {
                                            ui.horizontal_wrapped(|ui|{
                                                ui.label(line);
                                                let but = ui.button("удалить");
                                                if but.clicked() {
                                                    let del = delete_line(counter, self.sells.task.clone());
                                                    std::fs::remove_file(&self.sells.task);
                                                    let mut task = File::create(&self.sells.task.clone()).unwrap();
                                                    task.write_all(del.as_bytes());
                                                };
                                            });
                                            counter += 1;
                                        };
                                    };
                                }
                                else if id == 1 {
                                    let mut counter = 0;
                                    for lines in read_lines(self.assembly.task.clone()) {
                                        for line in lines {
                                            ui.horizontal_wrapped(|ui|{
                                                ui.label(line);
                                                let but = ui.button("удалить");
                                                if but.clicked() {
                                                    let del = delete_line(counter, self.assembly.task.clone());
                                                    std::fs::remove_file(&self.assembly.task);
                                                    let mut task = File::create(&self.assembly.task.clone()).unwrap();
                                                    task.write_all(del.as_bytes());
                                                };
                                            });
                                            counter += 1;
                                        };
                                        // let mut s = "".to_string()
                                        // ui.text_edit_singleline(&mut s);
                                        // if ui.button("добавить задачу").clicked() {
                                        //     //write_into();
                                        // }
                                    };
                                }
                                else {
                                    let mut counter = 0;
                                    for lines in read_lines(self.program.task.clone()){
                                        for line in lines {
                                            ui.horizontal_wrapped(|ui|{
                                                ui.label(line);
                                                let but = ui.button("удалить");
                                                let but = ui.button("удалить");
                                                if but.clicked() {
                                                    let del = delete_line(counter, self.program.task.clone());
                                                    std::fs::remove_file(&self.program.task);
                                                    let mut task = File::create(&self.program.task.clone()).unwrap();
                                                    task.write_all(del.as_bytes());
                                                };
                                            });
                                            counter += 1;
                                        };
                                    };
                                }
                            });
                        });
                        let _col = egui::collapsing_header::CollapsingHeader::new("Сотрудники")
                        .id_source(id)
                        .show(ui, |ui| {
                            if id == 0 {
                                let mut lab = "".to_string();
                                File::open(&self.sells.empl).expect(&self.sells.empl).read_to_string(&mut lab);
                                ui.label(format!("{}",lab));
                            }
                            else if id == 1 {
                                let mut lab = "".to_string();
                                File::open(&self.assembly.empl).expect(&self.assembly.empl).read_to_string(&mut lab);
                                ui.label(format!("{}",lab));
                            }
                            else {
                                let mut lab = "".to_string();
                                File::open(&self.program.empl).expect(&self.program.empl).read_to_string(&mut lab);
                                ui.label(format!("{}",lab));
                            };
                        });
                    });
                };
            });
        });
    }
}
