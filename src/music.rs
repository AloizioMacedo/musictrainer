pub struct Note {
    pub midi: u8,
}

pub fn generate_note() -> Note {
    use rand::Rng;
    let mut rng = rand::thread_rng();
    let midi = rng.gen_range(60u8..=81u8);
    Note { midi }
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

pub fn midi_to_staff_position(midi: u8) -> i16 {
    let pitch_class = midi % 12;
    let octave = (midi / 12) as i16 - 1;

    let natural_base = match pitch_class {
        0 | 1 => -2,
        2 | 3 => -1,
        4 => 0,
        5 | 6 => 1,
        7 | 8 => 2,
        9 | 10 => 3,
        11 => 4,
        _ => 0,
    };

    natural_base + 7 * (octave - 4)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_positioning() {
        assert_eq!(midi_to_staff_position(60), -2);
        assert_eq!(midi_to_staff_position(64), 0);
        assert_eq!(midi_to_staff_position(77), 8);
        assert_eq!(midi_to_staff_position(72), 5);
    }
}