use rizum::Timing;

#[test]
fn public_timing_api_converts_ticks_beats_and_seconds() {
    let timing = Timing {
        bpm: 120.0,
        ticks_per_beat: 480,
    };

    assert_eq!(timing.ticks_to_beats(480), 1.0);
    assert_eq!(timing.ticks_to_beats(240), 0.5);
    assert_eq!(timing.beats_to_seconds(1.0), 0.5);
    assert_eq!(timing.beats_to_seconds(2.0), 1.0);
}
