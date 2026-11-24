mod app;

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions {
        viewport: eframe::egui::ViewportBuilder::default()
            .with_inner_size([360.0, 420.0])
            .with_min_inner_size([320.0, 360.0]),
        ..Default::default()
    };

    eframe::run_native(
        "Pomodoro Timer",
        options,
        Box::new(|_creation_context| Ok(Box::<app::PomodoroApp>::default())),
    )
}
