use crate::music::{Note, generate_note};

const LETTER_TO_PC: [u8; 7] = [9, 11, 0, 2, 4, 5, 7];

pub struct App {
    pub current_note: Note,
    pub score: u32,
    pub streak: u32,
    pub feedback_text: String,
    pub feedback_ticks: u8,
    pub quit: bool,
}

impl App {
    pub fn new() -> Self {
        Self {
            current_note: generate_note(),
            score: 0,
            streak: 0,
            feedback_text: String::new(),
            feedback_ticks: 0,
            quit: false,
        }
    }

    pub fn answer(&mut self, letter: char) {
        let pc = LETTER_TO_PC[(letter.to_ascii_uppercase() as u8 - b'A') as usize];
        if pc == self.current_note.pitch_class {
            self.score += 1;
            self.streak += 1;
            self.feedback_text = format!("Correct! (+{})", 1 + self.streak / 5);
            self.feedback_ticks = 30;
            self.current_note = generate_note();
        } else {
            self.streak = 0;
            self.feedback_text = format!("{} is incorrect!", letter.to_ascii_uppercase());
            self.feedback_ticks = 40;
        }
    }

    pub fn tick(&mut self) {
        if self.feedback_ticks > 0 {
            self.feedback_ticks -= 1;
            if self.feedback_ticks == 0 {
                self.feedback_text.clear();
            }
        }
    }
}
