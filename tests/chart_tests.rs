use rizum::Chart;

#[test]
fn example_chart_round_trips_through_json() {
    let source = include_str!("../charts/example.json");
    let chart: Chart = serde_json::from_str(source).expect("example chart should deserialize");

    assert_eq!(chart.version, 1);
    assert_eq!(chart.metadata.title, "Example");
    assert_eq!(chart.metadata.artist, "Blasily");
    assert_eq!(chart.timing.bpm, 120.0);
    assert_eq!(chart.timing.ticks_per_beat, 480);
    let notes: Vec<_> = chart
        .notes
        .iter()
        .map(|note| (note.tick, note.lane))
        .collect();
    assert_eq!(notes, [(0, 0), (480, 1), (960, 2), (1440, 3)]);

    let serialized = serde_json::to_string(&chart).expect("chart should serialize");
    let mut original_value: serde_json::Value = serde_json::from_str(source).unwrap();
    let serialized_value: serde_json::Value = serde_json::from_str(&serialized).unwrap();
    // `bpm` is modeled as an f32, so serialization writes 120.0 even though
    // the example uses the equivalent JSON integer 120.
    original_value["timing"]["bpm"] = serde_json::json!(120.0);
    assert_eq!(serialized_value, original_value);
}

#[test]
fn invalid_or_incomplete_json_is_rejected() {
    assert!(serde_json::from_str::<Chart>("not json").is_err());
    assert!(serde_json::from_str::<Chart>(r#"{"version":1}"#).is_err());
}
