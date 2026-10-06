use eframe::egui;

pub struct MyApp {
    name: String,
    messages: Vec<String>,
    next_id: usize,
}

// pub struct ScrollAreaState {
//     offset: Vec2, // positive is down/right
//     max_offset: Vec2, // max amount you can scroll
//     offset_target: [Option<ScrollingToTarget>; 2], // the target to which to scroll to
//     show_scroll: Vec2b,
//     content_is_too_large: Vec2b,
//     scroll_bar_interaction: Vec2b,
//
//
// }

impl Default for MyApp {
    fn default() -> Self {
        Self {
            name: "name".to_string(),
            messages: vec![
                "Message 1".to_string(),
                "Message 2".to_string(),
                "Message 3".to_string(),
                "Message 4".to_string(),
                "Message 5".to_string(),
                "Message 6".to_string(),
                "Message 7".to_string(),
                "Message 8".to_string(),
                "Message 9".to_string(),
                "Message 10".to_string(),
                "Message 11".to_string(),
                "Message 12".to_string(),
            ],
            next_id: 15,
        }
    }
}

impl eframe::App for MyApp {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ui, |ui| {
            let mut message: String = "My message".to_string();

            ui.label(format!("Hello {}!", self.name));
            ui.heading("Decentralized Messenger");
            ui.label("At early stages of development... chat below!");

            ui.separator();

            // Scroll area with messages goes here
            egui::ScrollArea::vertical()
                .stick_to_bottom(true)
                .show(ui, |ui| {
                    for (i, message) in self.messages.iter().enumerate() {
                        ui.label(format!("[{}] {}", i + 1, message));
                    }
                });


            ui.horizontal(|ui| {
                ui.text_edit_singleline(&mut message);

                if ui.button("Send").clicked() {
                    println!("Message sent...");
                }
            });
        });
    }
}
