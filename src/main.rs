use crate::app::MyApp;

use std::error::Error;

mod app;
mod persistent_state;
mod app_state;

fn main() -> Result<(), Box<dyn Error>> {
    let options = eframe::NativeOptions {
        drag_and_drop_support: true,
        initial_window_size: Some([800.0, 600.0].into()),
        #[cfg(feature = "wgpu")]
        renderer: eframe::Renderer::Wgpu,
        ..Default::default()
    };
    let my_app = MyApp::default();
    eframe::run_native("egui template app", options, Box::new(|_cc| {
        Box::new(my_app)
    }));
    Ok(())
}
