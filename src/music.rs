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

pub fn staff_name(pos: i16) -> String {
    let idx = pos.rem_euclid(7) as usize;
    NATURAL_NAMES[idx].to_string()
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
}
