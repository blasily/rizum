use rizum::Chart;

#[test]
fn example_chart_round_trips_through_json() {
    let source = include_str!("../charts/example.json");
    let chart: Chart = serde_json::from_str(source).expect("example chart should deserialize");

    assert_eq!(chart.version, 1);
    assert_eq!(chart.metadata.title, "EXample");
    assert_eq!(chart.metadata.artist, "Blasily");
    assert_eq!(chart.timing.bpm, 120.0);
    assert_eq!(chart.timing.ticks_per_beat, 480);
    assert_eq!(chart.notes.len(), 4);
    assert_eq!(chart.notes[3].tick, 1440);
    assert_eq!(chart.notes[3].lane, 3);

    let serialized = serde_json::to_string(&chart).expect("chart should serialize");
    let original_value: serde_json::Value = serde_json::from_str(source).unwrap();
    let serialized_value: serde_json::Value = serde_json::from_str(&serialized).unwrap();
    assert_eq!(serialized_value, original_value);
}

#[test]
fn invalid_or_incomplete_json_is_rejected() {
    assert!(serde_json::from_str::<Chart>("not json").is_err());
    assert!(serde_json::from_str::<Chart>(r#"{"version":1}"#).is_err());
}
