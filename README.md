# Rizum

Rizum is an early-development Rust library for rhythm-game chart data. It currently provides public chart, metadata, timing, and note types, plus Serde JSON serialization and basic tick, beat, and time conversions. It is not yet a complete rhythm-game framework.

## Getting started

From the repository root, parse the checked-in example chart through the public API:

```rust
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let json = std::fs::read_to_string("charts/example.json")?;
    let chart: rizum::Chart = serde_json::from_str(&json)?;

    println!("{} by {}", chart.metadata.title, chart.metadata.artist);
    println!("{} BPM", chart.timing.bpm);

    for note in &chart.notes {
        let seconds = chart.timing.ticks_to_seconds(note.tick);
        println!("lane {} at tick {} ({seconds} seconds)", note.lane, note.tick);
    }

    Ok(())
}
```

The example assumes `rizum` and `serde_json` are available to the Rust program. The chart is read relative to the working directory, so run it from the repository root or adjust the path.

## Chart format

The chart is a JSON object with four required fields:

| Field | Rust type | Meaning |
| --- | --- | --- |
| `version` | `u32` | Chart format version (the example uses `1`). |
| `metadata` | `Metadata` | `title` and `artist` strings. |
| `timing` | `Timing` | A constant tempo and tick resolution for the chart. |
| `notes` | `Vec<Note>` | Notes, each with a `tick` and `lane`. |

`timing.bpm` is beats per minute. `timing.ticks_per_beat` is the number of ticks in one beat. A note's `tick` is its position from the start of the chart, in ticks. `lane` is stored as an integer; the example uses lane numbers 0 through 3. The crate does not define a lane count or validate lane assignments.

For example, the checked-in [`charts/example.json`](charts/example.json) uses 120 BPM and 480 ticks per beat. Its note at tick 480 is one beat from the start; tick 960 is two beats from the start.

## Timing conversions

`Timing` exposes `ticks_to_beats`, `beats_to_seconds`, and `ticks_to_seconds`. At 120 BPM with 480 ticks per beat:

- 480 ticks = 1 beat = 0.5 seconds.
- 960 ticks = 2 beats = 1 second.

```rust
let beats = chart.timing.ticks_to_beats(960); // 2.0
let seconds = chart.timing.ticks_to_seconds(960); // 1.0
let one_beat_seconds = chart.timing.beats_to_seconds(1.0); // 0.5
```

## Current limits

Each chart has one BPM and one ticks-per-beat value; tempo changes and timing offsets are not modeled. Notes have a tick and lane only, so durations and other note events are not represented. Timing values and lane assignments are not validated. In particular, conversion methods divide by `bpm` or `ticks_per_beat`, so those values should be positive and nonzero.

## Tests

Run the test suite with:

```sh
cargo test
```

The example chart is located at [`charts/example.json`](charts/example.json); integration tests under `tests/` deserialize it through the public `rizum::Chart` type.
