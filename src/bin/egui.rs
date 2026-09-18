use decen_messenger::gui::app::MessengerApp;

fn main() -> eframe::Result {
    eframe::run_native(
        "DecenMessenger",
        eframe::NativeOptions::default(),
        Box::new(|_cc| Ok(Box::new(MessengerApp::new()))),
    )
}
