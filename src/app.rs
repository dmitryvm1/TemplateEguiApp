use egui_file::FileDialog;
use crate::{persistent_state::PersistentState, app_state::AppState};

pub struct MyApp {
    file_dialog: FileDialog,
    ui_state: PersistentState<AppState>,
    path: Option<String>,
}

impl Default for MyApp {
    fn default() -> Self {
        let ui_state = PersistentState::<AppState>::load(None).unwrap();
        let path = ui_state.inner().recent_file.clone();
        Self {
            ui_state,
            file_dialog: FileDialog::open_file(path.map(std::path::PathBuf::from)),
            path: None,
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.ui(ctx);
    }
}

impl MyApp {
    pub fn file_selector(&mut self, ctx: &egui::Context, ui: &mut egui::Ui) {
        if ui.button("Open File").clicked() {
            self.path = None;
            self.file_dialog.open();
        }
        self.file_dialog.show(ctx);
        // Set path if no file is selected yet.
        if self.path.is_none() && self.file_dialog.selected() {
            let buf = self.file_dialog.path().unwrap();
            self.path = Some(buf.to_str().unwrap().to_owned());
            self.ui_state.inner_mut().recent_file = self.path.clone();
            self.ui_state.save(None).expect("failed to save ui state to disk");
        }
        if let Some(p) = &self.path {
            ui.label(format!("You have selected a file: {}", p));
        }
    }

    pub fn ui(&mut self, ctx: &egui::Context) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("My egui Application");
            self.file_selector(ctx, ui);
        });
    }
}
