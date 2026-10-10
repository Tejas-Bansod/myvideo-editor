# AI Engineering Rules — StudioGrade NLE

## 1. Zero-Crash Guarantee
- Never use `unwrap()`, `expect()`, `panic!`, `todo!`, or `unimplemented!` in production crates.
- Every function with potential failure must return `Result<T, E>` using `thiserror`.
- All inputs (files, user numbers, stream metadata) are untrusted. Use `.get()`, checked math (`checked_add`, `saturating_mul`), and guard against zero divisors and NaN values.
- Background worker threads must be wrapped in `std::panic::catch_unwind` and communicate errors across `crossbeam-channel` or `std::sync::mpsc`.

## 2. Pure Rust Architecture
- All components (UI, engine, codecs, render graph, persistence) must be 100% native Rust.
- Frontend: `eframe 0.36` + `egui 0.36` over `wgpu`.
- Engine & Workers: `rayon 1.12`, `serde`, `serde_json`, `rfd`.
- Separation of Concerns: Core engine crates (`timeline`, `color`, `audio`, `project`, `media`, `render`, `export`) must never depend on UI crates (`egui`, `eframe`).

## 3. Mathematical & Color Science Invariants
- Timeline and playback timebases must strictly use 64-bit rational time (`RationalTime { frames: i64, timebase: (u32, u32) }`). Never use floating-point seconds for edit calculations.
- Color transformations must operate in unclipped 32-bit floating-point linear space (ACEScct / ACEScg).

## 4. Atomic Persistence
- Projects (`.vedit`) must be serialized via `serde_json` to a temporary sibling file (`.vedit.tmp`) and replaced atomically using `std::fs::rename`.
