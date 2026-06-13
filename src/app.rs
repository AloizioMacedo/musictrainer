use crate::music::{generate_note, Note};

pub struct App {
    pub current_note: Note,
    pub user_input: String,
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
            user_input: String::new(),
            score: 0,
            streak: 0,
            feedback_text: String::new(),
            feedback_ticks: 0,
            quit: false,
        }
    }

    pub fn type_char(&mut self, c: char) {
        if self.user_input.is_empty() && c == 'b' {
            return;
        }
        self.user_input.push(c);
    }

    pub fn backspace(&mut self) {
        self.user_input.pop();
    }

    pub fn submit(&mut self) {
        if self.user_input.is_empty() {
            return;
        }
        let input = &self.user_input;
        let input_pitch_class = crate::music::parse_note(input);

        if let Some(pc) = input_pitch_class {
            let current_pc = self.current_note.midi % 12;
            if pc == current_pc {
                self.score += 1;
                self.streak += 1;
                self.feedback_text = format!("Correct! (+{})", 1 + self.streak / 5);
                self.feedback_ticks = 30;
                self.current_note = generate_note();
                self.user_input.clear();
            } else {
                self.streak = 0;
                self.feedback_text = format!("{} is wrong, try again", Self::pc_name(pc));
                self.feedback_ticks = 40;
            }
        } else {
            self.streak = 0;
            self.feedback_text = format!("\"{}\" is not a valid note", input);
            self.feedback_ticks = 40;
        }
    }

    fn pc_name(pc: u8) -> &'static str {
        match pc {
            0 => "C",
            1 => "C#/Db",
            2 => "D",
            3 => "D#/Eb",
            4 => "E",
            5 => "F",
            6 => "F#/Gb",
            7 => "G",
            8 => "G#/Ab",
            9 => "A",
            10 => "A#/Bb",
            11 => "B",
            _ => "?",
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