fn main() {
    engine::start(
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
        engine_context: &mut engine::EngineContext,
        ui_context: &engine::egui::Context,
    ) {
        let mut messages = Vec::new();
        engine::egui::Window::new("Demo").show(ui_context, |ui| {
            ui.heading("Spectral Engine Demo App");
            if ui.button("Click me!").clicked() {
                messages.push(engine::EngineMessage::Empty);
            }
        });
        messages.drain(..).for_each(|message| {
            engine_context.pending_messages.push(message);
        });
    }
}
