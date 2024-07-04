fn main() {
    engine::start(
        Demo::default(),
        engine::LaunchSettings {
            window_title: "Spectral Engine - Demo".to_string(),
        },
    );
}

#[derive(Default)]
pub struct Demo {
    client: Option<engine::EngineClient>,
}

#[async_trait::async_trait]
impl engine::State for Demo {
    async fn update(
        &mut self,
        engine_context: &mut engine::EngineContext,
        ui_context: &engine::egui::Context,
    ) {
        if self.client.is_none() {
            self.client = Some(engine::EngineClient::new(engine_context.broker.clone()));
        }

        let mut messages = Vec::new();
        engine::egui::Window::new("Demo").show(ui_context, |ui| {
            ui.heading("Spectral Engine Demo App");
            if ui.button("Click me!").clicked() {
                messages.push(engine::EngineMessage::Empty);
            }
        });
        for message in messages.drain(..) {
            if let Some(client) = self.client.as_ref() {
                client
                    .publish(&engine::EngineMessage::empty_topic(), message)
                    .await;
            }
        }
    }
}
