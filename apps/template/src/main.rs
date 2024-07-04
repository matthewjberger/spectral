fn main() {
    engine::launch(
        App,
        engine::LaunchSettings {
            window_title: "Spectral Engine - Template".to_string(),
        },
    );
}

#[derive(Default)]
pub struct App;

#[async_trait::async_trait]
impl engine::State for App {
    async fn update(
        &mut self,
        _engine_context: &mut engine::LaunchContext,
        ui_context: &engine::egui::Context,
    ) {
        engine::egui::Window::new("Template").show(ui_context, |ui| {
            ui.heading("Template App");
        });
    }
}
