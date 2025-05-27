use app::WordleApp;

mod game;
mod word;
mod app;

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "Rust Wordle",
        options,
        Box::new(|creation_context| 
            {
                creation_context.egui_ctx.set_theme(egui::Theme::Dark);
                Ok(Box::new(WordleApp::default()))
            }
        ),
    )
}