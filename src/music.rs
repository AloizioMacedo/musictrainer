const NATURAL_PITCH_CLASSES: [u8; 7] = [4, 5, 7, 9, 11, 0, 2];

const NATURAL_NAMES: [&str; 7] = ["E", "F", "G", "A", "B", "C", "D"];

pub struct Note {
    pub staff_position: i16,
    pub pitch_class: u8,
}

pub fn generate_note() -> Note {
    use rand::Rng;
    let mut rng = rand::thread_rng();
    let staff_position = rng.gen_range(-2i16..=10);
    let pitch_class = staff_position_to_pitch_class(staff_position);
    Note {
        staff_position,
        pitch_class,
    }
}

pub fn staff_position_to_pitch_class(pos: i16) -> u8 {
    let idx = pos.rem_euclid(7) as usize;
    NATURAL_PITCH_CLASSES[idx]
}

pub fn staff_position_name(pos: i16) -> String {
    let idx = pos.rem_euclid(7) as usize;
    let octave = 5 + (pos - 5).div_euclid(7);
    format!("{}{}", NATURAL_NAMES[idx], octave)
}

pub fn parse_note(input: &str) -> Option<u8> {
    let trimmed = input.trim().to_uppercase();

    let (letter, accidental) = if let Some(rest) = trimmed.strip_prefix('C') {
        ('C', rest)
    } else if let Some(rest) = trimmed.strip_prefix('D') {
        ('D', rest)
    } else if let Some(rest) = trimmed.strip_prefix('E') {
        ('E', rest)
    } else if let Some(rest) = trimmed.strip_prefix('F') {
        ('F', rest)
    } else if let Some(rest) = trimmed.strip_prefix('G') {
        ('G', rest)
    } else if let Some(rest) = trimmed.strip_prefix('A') {
        ('A', rest)
    } else if let Some(rest) = trimmed.strip_prefix('B') {
        ('B', rest)
    } else {
        return None;
    };

    let semitone = match letter {
        'C' => 0,
        'D' => 2,
        'E' => 4,
        'F' => 5,
        'G' => 7,
        'A' => 9,
        'B' => 11,
        _ => return None,
    };

    let offset: i8 = match accidental {
        "" | "N" => 0,
        "#" | "♯" => 1,
        "B" | "♭" => -1,
        "##" | "♯♯" => 2,
        "X" => 2,
        "BB" | "♭♭" => -2,
        _ => return None,
    };

    let pitch_class = (semitone as i8 + offset).rem_euclid(12) as u8;

    Some(pitch_class)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_staff_position_to_pitch_class() {
        assert_eq!(staff_position_to_pitch_class(-2), 0);
        assert_eq!(staff_position_to_pitch_class(0), 4);
        assert_eq!(staff_position_to_pitch_class(5), 0);
        assert_eq!(staff_position_to_pitch_class(8), 5);
    }

    #[test]
    fn test_staff_position_name() {
        assert_eq!(staff_position_name(0), "E4");
        assert_eq!(staff_position_name(5), "C5");
        assert_eq!(staff_position_name(-2), "C4");
        assert_eq!(staff_position_name(8), "F5");
    }
}
