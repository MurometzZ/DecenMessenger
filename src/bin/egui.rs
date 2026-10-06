use decen_messenger::gui::app::MyApp;
use eframe::egui;

fn main() -> eframe::Result {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([800.0, 200.0]),
        ..Default::default()
    };

    eframe::run_native(
        "Decen Messenger",
        options,
        Box::new(|_cc| Ok(Box::new(MyApp::default()))),
    )
}
