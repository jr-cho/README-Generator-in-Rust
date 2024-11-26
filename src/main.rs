use eframe::egui;
use rfd::FileDialog;
use std::path::PathBuf;

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "README Generator",
        options,
        Box::new(|_cc| Box::new(ReadmeApp::default())),
    )
}

struct ReadmeApp {
    title: String,
    description: String,
    installation: String,
    usage: String,
    github_username: String,
    status: String,
    save_path: Option<PathBuf>,
}

impl Default for ReadmeApp {
    fn default() -> Self {
        Self {
            title: String::new(),
            description: String::new(),
            installation: String::new(),
            usage: String::new(),
            github_username: String::new(),
            status: String::from("Awaiting input..."),
            save_path: None,
        }
    }
}

impl eframe::App for ReadmeApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let mut visuals = ctx.style().visuals.clone();
        visuals.widgets.inactive.rounding = egui::Rounding::same(0.0);
        visuals.widgets.active.rounding = egui::Rounding::same(0.0);
        visuals.widgets.hovered.rounding = egui::Rounding::same(0.0);
        let mut style = (*ctx.style()).clone();
        style.visuals = visuals;
        ctx.set_style(style);

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("✨ README Generator ✨");
            ui.separator();

            ui.add_space(5.0);
            ui.label("Project Title:");
            ui.text_edit_singleline(&mut self.title);
            ui.add_space(5.0);

            ui.label("Description:");
            ui.text_edit_multiline(&mut self.description);
            ui.add_space(5.0);

            ui.label("Installation Instructions:");
            ui.text_edit_multiline(&mut self.installation);
            ui.add_space(5.0);

            ui.label("Usage Instructions:");
            ui.text_edit_multiline(&mut self.usage);
            ui.add_space(5.0);

            ui.label("GitHub Username:");
            ui.text_edit_singleline(&mut self.github_username);
            ui.add_space(5.0);

            if ui.button("📁 Select Save Path").clicked() {
                if let Some(path) = FileDialog::new().pick_folder() {
                    self.save_path = Some(path);
                }
            }

            if let Some(ref path) = self.save_path {
                ui.label(format!("Save Path: {}", path.display()));
            } else {
                ui.label("No save path selected.");
            }

            ui.add_space(5.0);

            if ui
                .button(
                    egui::RichText::new("🚀 Generate README")
                        .size(18.0)
                        .color(egui::Color32::WHITE),
                )
                .clicked()
            {
                if let Some(ref save_path) = self.save_path {
                    let content = format!(
                        "# {}\n\n## Description\n{}\n\n## Installation\n```\n{}\n```\n\n## Usage\n```\n{}\n```\n\n## More\nYou can find more of my work at [{}](https://github.com/{})",
                        self.title.trim(),
                        self.description.trim(),
                        self.installation.trim(),
                        self.usage.trim(),
                        self.github_username.trim(),
                        self.github_username.trim()
                    );

                    let file_path = save_path.join("README.md");
                    match std::fs::write(&file_path, &content) {
                        Ok(_) => self.status = format!("README.md generated successfully at {}!", file_path.display()),
                        Err(e) => self.status = format!("Failed to create README.md: {}", e),
                    }
                } else {
                    self.status = "Please select a save path first.".to_string();
                }
            }

            ui.separator();
            ui.label(&self.status);
        });
    }
}
