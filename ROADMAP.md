# Rizum Roadmap

Rizum aims to be an engine-agnostic, headless rhythm-game framework written in Rust. The goal is to provide reusable chart, timing, and gameplay logic that a host application can use without tying Rizum to a specific game engine.

Rizum is at a very early stage. This roadmap describes a possible order of work, not delivery dates or firm commitments. Priorities may change as the core model and its users become clearer.

## Current state

The crate currently provides public chart, metadata, timing, and note types; JSON serialization and deserialization; and basic tick, beat, and time conversions. Charts use one BPM and one ticks-per-beat value, and notes currently contain only a tick and lane. The crate does not yet provide a gameplay session or judgement system.

## Near term: reliable chart and timing foundations

- Define chart validation and useful errors for invalid timing values, unsupported versions, and invalid note data.
- Clarify chart and lane semantics, including format-version compatibility expectations.
- Evolve timing support when the model is ready for tempo changes, offsets, and other timing events.
- Refine note and chart data types for common rhythm-game chart needs while keeping the format engine-agnostic.
- Keep examples, documentation, and integration tests aligned with the public data model.

## Next: headless gameplay runtime

- Design a deterministic play-session API that consumes chart data and time supplied by the host.
- Represent player inputs as timestamped events, without requiring a particular input device, renderer, audio backend, or engine loop.
- Add configurable judgement windows and expose note judgements and session results through Rust types and events.
- Test gameplay logic independently of real-time hardware and engine integrations.

## Later: stable Rust integration surface

- Stabilize the core Rust APIs and document compatibility expectations.
- Provide examples showing how a host application can drive the headless runtime and respond to its events.
- Review ownership, error reporting, and threading choices before committing to a foreign-function interface.

## Long term: C ABI and official C# binding

Once the Rust API and core runtime have settled, explore a versioned C ABI and an official C# binding. This work should define clear handle lifetimes, memory ownership, error behavior, and supported platforms. Interoperability is a long-term direction, not an early-development milestone.

## Out of scope for the core

Rizum is intended to remain independent of any one game engine. Engine-specific rendering, audio, and device input integrations can be built by host applications or separate adapters; the core should expose the data and gameplay behavior those integrations need.
