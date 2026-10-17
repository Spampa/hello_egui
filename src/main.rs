use eframe::egui::{self, RichText};
use std::{
    error::Error,
    fs::File,
    io::{BufRead, BufReader, BufWriter, Write},
    path::Path,
    process, thread,
};

fn main() -> eframe::Result {
    let options = eframe::NativeOptions::default();

    let list = ToDoApp::load_data().unwrap_or_else(|err| {
        eprintln!("Failed to load data: {}", err);
        process::exit(1);
    });

    eframe::run_native(
        "ToDo List",
        options,
        Box::new(|_cc| Ok(Box::new(ToDoApp::new(list)))),
    )
}

#[derive(Clone)]
struct ToDo {
    name: String,
    description: Option<String>,
    is_completed: bool,
}

impl ToDo {
    fn ui(&mut self, ui: &mut egui::Ui) {
        let mut title = RichText::new(self.name.clone()).size(24.0).strong();
        let description = RichText::new(self.description.clone().unwrap_or_default())
            .size(16.0)
            .small_raised();

        if self.is_completed {
            title = title.strikethrough();
        }

        ui.vertical(|ui| {
            ui.checkbox(&mut self.is_completed, title);
            ui.horizontal(|ui| {
                let indent = ui.spacing().icon_width + ui.spacing().icon_spacing;
                ui.add_space(indent);
                ui.label(description);
            });
        });
    }
}

#[derive(Default)]
struct ToDoApp {
    list: Vec<ToDo>,
    new_name: String,
    new_description: String,
}

impl ToDoApp {
    pub fn new(list: Vec<ToDo>) -> Self {
        Self {
            list,
            ..Default::default()
        }
    }

    fn save_data(list: &Vec<ToDo>) -> Result<(), Box<dyn Error>> {
        let mut writer = BufWriter::new(File::create("data.csv")?);
        for todo in list {
            let line = format!(
                "{};{};{}\n",
                todo.name,
                todo.description.clone().unwrap_or_default(),
                todo.is_completed.to_string()
            );

            writer.write_all(line.as_bytes())?;
        }
        writer.flush()?;
        Ok(())
    }

    fn load_data() -> Result<Vec<ToDo>, Box<dyn Error>> {
        if !Path::new("data.csv").try_exists()? {
            return Ok(vec![]);
        }

        let reader = BufReader::new(File::open("data.csv")?);
        let mut list = vec![];

        for line in reader.lines() {
            let line = line?;
            let fields: Vec<&str> = line.split(';').collect();

            list.push(ToDo {
                name: fields[0].to_string(),
                description: Some(fields[1].to_string()),
                is_completed: fields[2] == "true",
            });
        }

        Ok(list)
    }

    fn create_form(&mut self, ui: &mut egui::Ui) {
        ui.heading("New ToDo");
        egui::Grid::new("create_form")
            .num_columns(2)
            .show(ui, |ui| {
                ui.label("Name:");
                ui.text_edit_singleline(&mut self.new_name);
                ui.end_row();

                ui.label("Description:");
                ui.text_edit_multiline(&mut self.new_description);
                ui.end_row();
            });

        let name = self.new_name.trim();
        if ui
            .add_enabled(!name.is_empty(), egui::Button::new("Add"))
            .clicked()
        {
            let description = self.new_description.trim();
            self.list.push(ToDo {
                name: name.to_string(),
                description: (!description.is_empty()).then(|| description.to_string()),
                is_completed: false,
            });
            self.new_name.clear();
            self.new_description.clear();
        }
    }
}

impl eframe::App for ToDoApp {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ui, |ui| {
            self.create_form(ui);
            ui.separator();

            let mut to_remove = None;

            for (i, todo) in self.list.iter_mut().enumerate() {
                ui.horizontal(|ui| {
                    todo.ui(ui);
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        if ui.button("Delete").clicked() {
                            to_remove = Some(i);
                        }
                    });
                });
                ui.separator();
            }

            if let Some(i) = to_remove {
                self.list.remove(i);
            }

            if ui.button("Save").clicked() {
                let list = self.list.clone();
                thread::spawn(move || {
                    if let Err(e) = Self::save_data(&list) {
                        eprintln!("Save failed: {e}");
                    }
                });
            }
        });
    }
}
