fn main() {
    engine::launch(
        Editor,
        engine::LaunchSettings {
            window_title: "Spectral Engine".to_string(),
        },
    );
}

#[derive(Default)]
pub struct Editor;

#[async_trait::async_trait]
impl engine::State for Editor {
    async fn update(
        &mut self,
        _engine_context: &mut engine::LaunchContext,
        ui_context: &engine::egui::Context,
    ) {
        #[cfg(not(target_arch = "wasm32"))]
        let title = "Rust/Wgpu";

        #[cfg(feature = "webgpu")]
        let title = "Rust/Wgpu/Webgpu";

        #[cfg(feature = "webgl")]
        let title = "Rust/Wgpu/Webgl";

        engine::egui::Window::new(title).show(ui_context, |ui| {
            ui.heading("Hello, world!");
            if ui.button("Click me!").clicked() {
                engine::log::info!("Button clicked!");
            }
        });
    }
}
