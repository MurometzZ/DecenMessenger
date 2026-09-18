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
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("DecenMessenger");

            ui.separator();

            ui.text_edit_singleline(&mut self.message);

            if ui.button("Send").clicked() {
                println!("Message: {}", self.message);
                self.message.clear();
            }
        });
    }
}
