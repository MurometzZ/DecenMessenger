use eframe::egui;

pub struct MessengerApp {
    message: String,
}

impl MessengerApp {
    pub fn new() -> Self {
        Self {
            message: String::new(),
        }
    }
}

impl eframe::App for MessengerApp {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        ui.heading("DecenMessenger");

        ui.separator();

        ui.horizontal(|ui| {
            ui.text_edit_singleline(&mut self.message);

            if ui.button("Send").clicked() {
                println!("Message: {}", self.message);
                self.message.clear();
            }
        });
    }
}
