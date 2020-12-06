#[derive(Debug, Clone, PartialEq)]
pub struct Questionaire {
    answers: [bool; 26],
}

impl Questionaire {
    pub fn new(text: &str) -> Questionaire {
        let mut answers: [bool; 26] = [false; 26];

        for ch in text.chars().collect::<Vec<_>>() {
            if let Ok(index) = Self::letter_to_index(ch) {
                answers[index] = true;
            }
        }

        Questionaire { answers }
    }

    fn letter_to_index(letter: char) -> Result<usize, String> {
        for (key, ch) in ('a'..='z').collect::<Vec<_>>().iter().enumerate() {
            if *ch == letter {
                return Ok(key);
            }
        }

        Err(format!("Unrecognized question letter given: {}.", letter))
    }

    pub fn get_answers(&self) -> &[bool; 26] {
        &self.answers
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct QuestionaireGroup {
    questionaires: Vec<Questionaire>,
}

impl QuestionaireGroup {
    pub fn new(text: &str) -> QuestionaireGroup {
        let mut questionaires = Vec::new();
        for questionaire_text in text.split_whitespace().collect::<Vec<_>>() {
            questionaires.push(Questionaire::new(questionaire_text));
        }

        QuestionaireGroup { questionaires }
    }

    pub fn sum_of_any_yes(&self) -> usize {
        let mut total_answers: [bool; 26] = [false; 26];

        for questionaire in &self.questionaires {
            for (key, &val) in questionaire.answers.iter().enumerate() {
                total_answers[key] = total_answers[key] || val;
            }
        }

        let mut total = 0;
        for answer in total_answers.iter() {
            total += if *answer { 1 } else { 0 };
        }
        total
    }

    pub fn sum_of_all_yes(&self) -> usize {
        let mut total_answers: [bool; 26] = [true; 26];

        for questionaire in &self.questionaires {
            for (key, &val) in questionaire.get_answers().iter().enumerate() {
                total_answers[key] = total_answers[key] && val;
            }
        }

        let mut total = 0;
        for answer in total_answers.iter() {
            total += if *answer { 1 } else { 0 };
        }
        total
    }
}
