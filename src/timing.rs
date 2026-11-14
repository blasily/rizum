use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Timing {
    pub bpm: f32,
    pub ticks_per_beat: u16,
}
}

pub fn beats_to_seconds(beats: f32, bpm: f32) -> f32 {
    return (beats / bpm) * 60.0;
}

pub fn ticks_to_seconds(ticks: u32, ticks_per_beat: u16, bpm: f32) -> f32 {
    return ((ticks as f32 / ticks_per_beat as f32) / bpm) * 60.0;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ticks_to_beats_works() {
        assert_eq!(ticks_to_beats(480, 480), 1.0);
        assert_eq!(ticks_to_beats(240, 480), 0.5);
        assert_eq!(ticks_to_beats(960, 480), 2.0);
    }

    #[test]
    fn beats_to_seconds_works() {
        assert_eq!(beats_to_seconds(1.0, 120.0), 0.5);
        assert_eq!(beats_to_seconds(2.0, 120.0), 1.0);
    }

    #[test]
    fn ticks_to_seconds_works() {
        assert_eq!(ticks_to_seconds(480, 480, 120.0), 0.5);
        assert_eq!(ticks_to_seconds(240, 480, 120.0), 0.25);
    }
}
