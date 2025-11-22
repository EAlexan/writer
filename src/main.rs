use writer::MyApp;

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "Writer",
        options,
        Box::new(|_cc| Ok(Box::new(MyApp::default()))),
    )
}