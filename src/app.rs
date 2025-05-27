use eframe::egui;
use crate::{game::{self, CharResult, Game, GameStatus}, word::Word};

pub struct WordleApp {
    game: Game,
    current_input: String,
    message: String,
}

impl Default for WordleApp {
    fn default() -> Self {
        Self {
            game: Game::random(),
            current_input: String::new(),
            message: String::from("Guess the 5-letter word!"),
        }
    }
}

impl eframe::App for WordleApp {
    fn update(&mut self, ctx: &egui::Context, _: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Rust Wordle");
            ui.label(&self.message);

            let input = ui.text_edit_singleline(&mut self.current_input);

            if input.lost_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter)) {
                if self.current_input.len() == 5 {
                    if let Ok(word) = Word::try_from(self.current_input.as_str()) {
                        if !self.game.is_guess_valid(&word) {
                            self.message = "Word not in valid word list".into();
                        }
                        else {
                            self.game.guess(word);
                            self.message = format!("Guess submitted: {}", self.current_input);
                            self.current_input.clear();
                        }
                    } else {
                        self.message = "Failed to parse word".into();
                    }
                } else {
                    self.message = "Word must be 5 characters".into();
                }
            }

            for round in 0..*self.game.round() {
            if let Some(result) = self.game.history()[round as usize] {
                let mut row = String::new();
                for res in result.result().iter() {
                    let emoji = match res {
                        CharResult::Green => "G",
                        CharResult::Yellow => "Y",
                        CharResult::None => "_",
                    };
                    row.push_str(emoji);
                }
                ui.label(row);
            }
        }

            if matches!(self.game.status(), GameStatus::Correct) {
                ui.label("You won!");
            } else if matches!(self.game.status(), GameStatus::Fail) {
                ui.label("You lost!");
            }
        });
    }
}