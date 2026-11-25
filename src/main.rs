mod app;

fn main() -> eframe::Result<()> {
    let icon = eframe::icon_data::from_png_bytes(include_bytes!("../assets/icon.png"))
        .expect("the embedded app icon must be a valid PNG");
    let options = eframe::NativeOptions {
        viewport: eframe::egui::ViewportBuilder::default()
            .with_inner_size([360.0, 420.0])
            .with_min_inner_size([320.0, 360.0])
            .with_icon(icon),
        ..Default::default()
    };

    eframe::run_native(
        "Pomodoro Timer",
        options,
        Box::new(|_creation_context| Ok(Box::<app::PomodoroApp>::default())),
    )
}
