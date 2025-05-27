use std::process::Output;

use crate::{game::{all_possible_results, WordResult}, word::Word};

pub fn new_possibility_space(context: WordResult, current_possibility_space: Vec<Word>) -> Vec<Word> {
    let mut output = vec![];

    for word in current_possibility_space {
        if good_result(word, context) {
            output.push(word);
        }
    }

    output
}

pub fn new_possibility_space_size(context: WordResult, current_possibility_space: &Vec<Word>) -> usize {
    let mut count = 0;

    for word in current_possibility_space {
        if good_result(*word, context) {
            count += 1;
        }
    }

    count
}

fn good_result(maybe_correct: Word, context: WordResult) -> bool {
    let input_word = context.input();
    let result = WordResult::compare_guess(*input_word, maybe_correct);
    return result.result() == context.result();
}

pub fn entropy(word: Word, current_possibility_space: &Vec<Word>) -> f32 {
    let mut sum = 0.0f32;

    for result in all_possible_results() {
        let context = WordResult::new(word, result);
        let new_space_size = new_possibility_space_size(context, &current_possibility_space);
        let p = current_possibility_space.len() as f32 / new_space_size as f32;

        if p != 0.0f32 {
            sum -= p * p.log2();
        }
    }

    sum
}

pub fn max_information(current_possibility_space: Vec<Word>) -> Word {
    let mut current_word = current_possibility_space[0];
    let mut current_max = f32::MIN;

    for word in &current_possibility_space {
        let entropy = entropy(*word, &current_possibility_space);
        if current_max < entropy {
            current_max = entropy;
            current_word = word.clone();
        }
    }

    current_word
}