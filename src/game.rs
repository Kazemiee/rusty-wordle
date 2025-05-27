use egui::output;
use rand::{rng, Rng};

use crate::word::Word;

pub struct Game {
    history: [Option<WordResult>; 6],
    round: u8,
    answer: Word,
    status: GameStatus,
    possible_words: Vec<Word>, 
}

pub enum GameStatus {
    Correct,
    Incomplete,
    Fail,
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum CharResult {
    Green,
    Yellow,
    None,
}

pub fn all_possible_results() -> [[CharResult; 5]; 243] {
    let mut output = [[CharResult::None; 5]; 243];

    for i in 0..243 {
        let mut n = i;
        for j in (0..5).rev() {
            output[i][j] = match n % 3 {
                0 => CharResult::None,
                1 => CharResult::Yellow,
                2 => CharResult::Green,
                _ => unreachable!(),
            };
            n /= 3;
        }
    }

    output
}

#[derive(Clone, Copy)]
pub struct WordResult {
    input: Word,
    result: [CharResult; 5],
}

impl WordResult {
    pub fn new(input: Word, answer: Word) -> Self {
        let mut result = [CharResult::None; 5];
        let mut count_unique = 0;
        let mut unique_in_input = [0u8; 5];

        'outer: for n in 0..5 {
            for i in 0..count_unique {
                if unique_in_input[i] == input[n] {
                    continue 'outer;
                }
            }
            unique_in_input[count_unique] = input[n];
            count_unique += 1;
        }

        for character in unique_in_input  {
            let (positions_ans, count_ans) = answer.positions_with_count(character);
            let (positions_in, count_in) = input.positions_with_count(character);

            let mut positions_green = [255; 5];
            let mut count_green = 0;
            let mut positions_non_green = [255; 5];
            let mut count_non_green = 0;

            for index_ans in 0..count_ans {
                for index_in in count_green..count_in {
                    let index_word_in = positions_in[index_in as usize];
                    let index_word_ans = positions_ans[index_ans as usize];
                    if index_word_in == index_word_ans {
                        result[index_word_in as usize] = CharResult::Green;
                        positions_green[count_green as usize] = index_in as u8;
                        count_green += 1;
                    }
                    else {
                        positions_non_green[count_non_green as usize] = index_in as u8;
                        count_non_green += 1;
                    }
                }
            }

            //  Yellows(char) = count_in_both(char) - Greens(char)

            let yellow_count = count_in.min(count_ans) - count_green;

            let mut current_non_green_index = 0;

            for _ in 0..yellow_count {
                let index_word_in = positions_in[current_non_green_index as usize];
                result[index_word_in as usize] = CharResult::Yellow;
                current_non_green_index += 1;
            }
        }

        Self { input: input, result: result }
    }

    pub fn result(&self) -> &[CharResult; 5] {
        &self.result
    }

    pub fn input(&self) -> &Word {
        &self.input
    }
}

fn possible_guesses() -> Vec<Word> {
    let file_data = include_str!("../valid-wordle-words.txt");
    let words = file_data.split("\n");

    let mut output: Vec<Word> = words.map(
        |value| Word::try_from(value).unwrap_or_else(|_| panic!("Failed to parse '{}' from valid wordle words", value))
    )
    .collect();

    output.sort();

    output
}

impl Game {
    pub fn new(answer: &str) -> Self {
        Game { 
            history: [None; 6], 
            round: 0, 
            answer: Word::try_from(answer).expect("Failed to parse answer as wordle word."), 
            status: GameStatus::Incomplete, 
            possible_words: possible_guesses()
        }
    }

    pub fn random() -> Self {
        let mut rng = rng();
        let possible_guesses = possible_guesses();
        Game { 
            history: [None; 6], 
            round: 0, 
            answer: possible_guesses[rng.random_range(0..possible_guesses.len())], 
            status: GameStatus::Incomplete, 
            possible_words: possible_guesses
        }
    }

    pub fn is_guess_valid(&self, input: &Word) -> bool {
        match self.possible_words.binary_search(input) {
            Ok(_) => true,
            Err(_) => false,
        }
    }

    pub fn possible_guesses(&self) -> &Vec<Word> {
        &self.possible_words
    }

    pub fn guess(&mut self, input: Word) -> () {
        let new_result = WordResult::new(input, self.answer);

        self.history[self.round as usize] = Some(new_result);
        self.round += 1;

        if input == self.answer {
            self.status = GameStatus::Correct;
        }
        else if self.round == 6 {
            self.status = GameStatus::Fail;
        }
    }

    pub fn history(&self) -> &[Option<WordResult>; 6] {
        &self.history
    }

    pub fn status(&self) -> &GameStatus {
        &self.status
    }

    pub fn round(&self) -> &u8 {
        &self.round
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use super::*;
    
    #[test]
    fn test_all_possible_results_unique() {
        let results = all_possible_results();
        let mut seen = HashSet::new();

        for combo in results.iter() {
            assert!(
                seen.insert(*combo),
                "Duplicate pattern found: {:?}",
                combo,
            );
        }

        assert_eq!(seen.len(), 243, "Expected 243 combinations");
    }

    #[test]
    fn test_all_green() {
        let guess = Word::try_from("apple").unwrap();
        let answer = Word::try_from("apple").unwrap();
        let result = WordResult::new(guess, answer);

        assert_eq!(
            result.result(),
            &[CharResult::Green; 5]
        );
    }

    #[test]
    fn test_all_none() {
        let guess = Word::try_from("abcde").unwrap();
        let answer = Word::try_from("fghij").unwrap();
        let result = WordResult::new(guess, answer);

        assert_eq!(
            result.result(),
            &[CharResult::None; 5]
        );
    }

    #[test]
    fn test_all_yellow() {
        let guess = Word::try_from("pleap").unwrap();
        let answer = Word::try_from("apple").unwrap();
        let result = WordResult::new(guess, answer);
        assert_eq!(
            result.result(),
            &[
                CharResult::Yellow,
                CharResult::Yellow,
                CharResult::Yellow,
                CharResult::Yellow,
                CharResult::Yellow,
            ]
        );
    }

    #[test]
    fn test_mixed_result() {
        let guess = Word::try_from("crane").unwrap();
        let answer = Word::try_from("candy").unwrap();
        let result = WordResult::new(guess, answer);

        assert_eq!(
            result.result(),
            &[
                CharResult::Green,
                CharResult::None,
                CharResult::Yellow,
                CharResult::Yellow,
                CharResult::None,
            ]
        );
    }
}