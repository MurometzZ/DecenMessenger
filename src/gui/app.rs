use eframe::egui;

fn main() -> eframe::Result {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([400.0, 300.0]),
        ..Default::default()
    };

    eframe::run_native(
        "Decen Messenger",
        options,
        Box::new(|_cc| Ok(Box::new(MyApp::default()))),
    )
}

pub struct MyApp {
    name: String,
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            name: "World".to_string(),
        }
    }
}

impl eframe::App for MyApp {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ui, |ui| {
            ui.heading("Privet world!");
            ui.label("Testing the gui build using egui");
            ui.label(format!("Hello {}!", self.name));
        });
    }
}
