fn main() {
    engine::launch(
        Demo,
        engine::LaunchSettings {
            window_title: "Spectral Engine - Demo".to_string(),
        },
    );
}

#[derive(Default)]
pub struct Demo;

impl engine::State for Demo {
    fn update(
        &mut self,
        _engine_context: &mut engine::LaunchContext,
        ui_context: &engine::egui::Context,
    ) {
        engine::egui::Window::new("Demo").show(ui_context, |ui| {
            ui.heading("Spectral Engine Demo App");
        });
    }
}
