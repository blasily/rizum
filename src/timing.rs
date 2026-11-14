use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Timing {
    pub bpm: f32,
    pub ticks_per_beat: u16,
}

impl Timing {
    pub fn ticks_to_beats(&self, ticks: u32) -> f32 {
        ticks as f32 / self.ticks_per_beat as f32
    }

    pub fn beats_to_seconds(&self, beats: f32) -> f32 {
        (beats / self.bpm) * 60.0
    }

    pub fn ticks_to_seconds(&self, ticks: u32) -> f32 {
        (self.ticks_to_beats(ticks) / self.bpm) * 60.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TIMING: Timing = Timing {
        bpm: 120.0,
        ticks_per_beat: 480,
    };

    #[test]
    fn ticks_to_beats_works() {
        assert_eq!(TIMING.ticks_to_beats(480), 1.0);
        assert_eq!(TIMING.ticks_to_beats(240), 0.5);
        assert_eq!(TIMING.ticks_to_beats(960), 2.0);
    }

    #[test]
    fn beats_to_seconds_works() {
        assert_eq!(TIMING.beats_to_seconds(1.0), 0.5);
        assert_eq!(TIMING.beats_to_seconds(2.0), 1.0);
    }

    #[test]
    fn ticks_to_seconds_works() {
        assert_eq!(TIMING.ticks_to_seconds(480), 0.5);
        assert_eq!(TIMING.ticks_to_seconds(240), 0.25);
    }
}
