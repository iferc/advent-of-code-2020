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
        // I would bet there is a smarter way to do this.. ¯\_(ツ)_/¯
        Ok(match letter {
            'a' => 0,
            'b' => 1,
            'c' => 2,
            'd' => 3,
            'e' => 4,
            'f' => 5,
            'g' => 6,
            'h' => 7,
            'i' => 8,
            'j' => 9,
            'k' => 10,
            'l' => 11,
            'm' => 12,
            'n' => 13,
            'o' => 14,
            'p' => 15,
            'q' => 16,
            'r' => 17,
            's' => 18,
            't' => 19,
            'u' => 20,
            'v' => 21,
            'w' => 22,
            'x' => 23,
            'y' => 24,
            'z' => 25,
            _ => return Err(format!("Unrecognized question letter given: {}.", letter)),
        })
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
