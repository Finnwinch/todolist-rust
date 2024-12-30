use eframe::egui;

struct TodoApp {
    todos: Vec<String>,
    new_todo: String,
}

impl Default for TodoApp {
    fn default() -> Self {
        Self {
            todos: vec![],
            new_todo: String::new(),
        }
    }
}

impl eframe::App for TodoApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("To-Do List");

            ui.horizontal(|ui| {
                ui.text_edit_singleline(&mut self.new_todo);
                if ui.button("Add").clicked() && !self.new_todo.trim().is_empty() {
                    self.todos.push(self.new_todo.trim().to_string());
                    self.new_todo.clear();
                }
            });

            ui.separator();

            let mut to_remove = Vec::new();
            for (i, todo) in self.todos.iter().enumerate() {
                ui.horizontal(|ui| {
                    ui.label(todo);
                    if ui.button("Delete").clicked() {
                        to_remove.push(i);
                    }
                });
            }

            for &i in to_remove.iter().rev() {
                self.todos.remove(i);
            }
        });
    }
}

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "To-Do List",
        options,
        Box::new(|_cc| Box::new(TodoApp::default())),
    )
}
