use eframe::egui;

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
        }
    }
}

impl eframe::App for ReadmeApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("README Generator");

            ui.horizontal(|ui| {
                ui.label("Title:");
                ui.text_edit_singleline(&mut self.title);
            });

            ui.horizontal(|ui| {
                ui.label("Description:");
                ui.text_edit_multiline(&mut self.description);
            });

            ui.horizontal(|ui| {
                ui.label("Installation Instructions:");
                ui.text_edit_multiline(&mut self.installation);
            });

            ui.horizontal(|ui| {
                ui.label("Usage Instructions:");
                ui.text_edit_multiline(&mut self.usage);
            });

            ui.horizontal(|ui| {
                ui.label("GitHub Username:");
                ui.text_edit_singleline(&mut self.github_username);
            });

            if ui.button("Generate README").clicked() {
                let content = format!(
                    "# {}\n\n## Description\n{}\n\n## Installation\n```\n{}\n```\n\n## Usage\n```\n{}\n```\n\n## More\nYou can find more of my work at [{}](https://github.com/{})",
                    self.title.trim(),
                    self.description.trim(),
                    self.installation.trim(),
                    self.usage.trim(),
                    self.github_username.trim(),
                    self.github_username.trim()
                );

                match std::fs::write("README.md", content) {
                    Ok(_) => self.status = "README.md generated successfully!".to_string(),
                    Err(e) => self.status = format!("Failed to create README.md: {}", e),
                }
            }

            ui.separator();
            ui.label(&self.status);
        });
    }
}
