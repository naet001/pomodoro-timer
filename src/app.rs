use eframe::egui;
use pomodoro_timer::timer::{Timer, TimerState};
use std::time::Instant;

pub struct PomodoroApp {
    timer: Timer,
}

impl Default for PomodoroApp {
    fn default() -> Self {
        Self {
            timer: Timer::new(),
        }
    }
}

impl eframe::App for PomodoroApp {
    fn update(&mut self, context: &egui::Context, _frame: &mut eframe::Frame) {
        let now = Instant::now();
        self.timer.update(now);

        egui::CentralPanel::default().show(context, |ui| {
            ui.vertical_centered(|ui| {
                ui.add_space(48.0);
                ui.heading("Pomodoro Timer");
                ui.add_space(24.0);

                ui.label(
                    egui::RichText::new(self.timer.phase().label())
                        .size(24.0)
                        .strong(),
                );
                ui.add_space(8.0);

                let remaining = self.timer.remaining().as_secs();
                ui.label(
                    egui::RichText::new(format!("{:02}:{:02}", remaining / 60, remaining % 60))
                        .size(72.0)
                        .monospace(),
                );

                match self.timer.state() {
                    TimerState::Ready => {
                        ui.label("Ready");
                    }
                    TimerState::Running => {
                        ui.label("In progress");
                    }
                    TimerState::Paused => {
                        ui.label("Paused");
                    }
                    TimerState::Complete => {
                        let next_phase = self
                            .timer
                            .next_phase()
                            .map(|phase| phase.label())
                            .unwrap_or("next phase");
                        ui.label(format!("Phase complete. Next: {next_phase}"));
                    }
                }

                ui.add_space(24.0);
                ui.horizontal(|ui| {
                    if self.timer.state() == TimerState::Running {
                        if ui.button("Pause").clicked() {
                            self.timer.pause(Instant::now());
                        }
                    } else if ui.button("Start").clicked() {
                        self.timer.start(Instant::now());
                    }

                    if ui.button("Reset").clicked() {
                        self.timer.reset();
                    }
                    if ui.button("Skip").clicked() {
                        self.timer.skip();
                    }
                });

                ui.add_space(16.0);
                ui.label(format!(
                    "Work sessions: {}",
                    self.timer.completed_work_sessions()
                ));
            });
        });

        if self.timer.state() == TimerState::Running {
            context.request_repaint_after(std::time::Duration::from_millis(200));
        }
    }
}
