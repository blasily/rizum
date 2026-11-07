pub fn ticks_to_beats(
    ticks: [rizum::Note::tick],
    ticks_per_beat: rizum::Chart::ticks_per_beat,
) -> [u32] {
    let mut beats: vec![];

    for tick in &ticks {
        let beat = tick / ticks_per_beat;

        beats.push(beat);
    }

    return beats;
}

pub fn beats_to_seconds(beats: [u32], bpm: rizum::Chart::bpm) -> [u32] {
    let mut final_seconds: [u32];

    for beat in &beats {
        let seconds = (beat / bpm) * 60;

        final_seconds.push(seconds);
    }

    return final_seconds;
}

pub fn ticks_to_seconds(
    ticks: [rizum::Note::tick],
    ticks_per_beat: rizum::Chart::ticks_per_beat,
    bpm: rizum::Chart::bpm,
) -> [u32] {
    let mut final_seconds: [u32];

    for tick in &ticks {
        let seconds = ((tick / ticks_per_beat) / bpm) * 60;

        final_seconds.push(seconds);
    }

    return final_seconds;
}
