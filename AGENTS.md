# StudioGrade NLE — Rules for AI Agents and Contributors

These rules govern all human and AI contributors working on **StudioGrade NLE**. They ensure the editor remains bulletproof, blistering fast, mathematically uncompromising, and a joy to develop with high-velocity **vibecoding**.

---

## 🛡️ Part I: AI Engineering & Safety Rules (Non-Negotiables)

### 0. Never Crash (Absolute Zero-Panic Policy)
Video editors handle hours of irreplaceable creative work. A malformed media file, a corrupt project JSON, an unexpected keystroke, or an out-of-bounds frame scrub must **never crash the process** or lose user state.
1. **Always Return `Result<T, E>`**: Propagate failures using `?` with descriptive error variants via `thiserror`. Never swallow context or silently corrupt projects.
2. **Strict Zero-Panic Ban**: Absolutely no `unwrap()`, `expect()`, `panic!`, `todo!`, or `unimplemented!` in production crate code. Use `?`, `ok_or()`, `let Some(x) = ... else { return ... }`, or safe fallbacks.
3. **Hostile Input Principle**: All external data is untrusted:
   - Index slices safely with `.get()` or check bounds once up front.
   - Use checked, saturating, or wrapping arithmetic (`checked_add`, `saturating_sub`).
   - Never divide by a variable that could be zero (frame rates, timebases, sample rates, scale factors).
   - Slice strings only along valid UTF-8 character boundaries.
   - Guard against negative or NaN values when casting floats to integers.
4. **Isolate Worker Threads**: All background jobs (decoding, proxy transcoding, waveform extraction, rendering) run under `std::panic::catch_unwind`. A worker failure must be reported cleanly over channels without crashing the application shell or poisoning the UI loop.
5. **Memory Safety & Unsafe Rules**: The workspace compiles under `#![deny(unsafe_code)]`. If low-level OS media hardware interop is required, it must be isolated behind safe trait abstractions with documented `// SAFETY:` justifications.

### 1. Pure Rust & Architecture Layering
1. **100% Pure Rust**: The entire stack—GUI, timeline engine, color science, decoders, audio pipeline, and export core—is native Rust. No JavaScript runtimes, no Python scripts, no JVM overhead.
2. **Strict Layering Discipline**:
   - `crates/timeline`, `crates/color`, `crates/audio`, `crates/project`, and `crates/media` must remain pure engine crates and **never depend on `egui`, `eframe`, or UI frameworks**.
   - `apps/desktop` consumes the engine crates and binds them to `eframe` and `egui`.
3. **Atomic Persistence**: Project saves must serialize to `.vedit.tmp` and commit via atomic file swap (`std::fs::rename`). Projects must never be written in-place to prevent partial write corruptions during unexpected power loss.

### 2. Mathematical Precision & Color Science Invariants
1. **Exact Rational Timebase**: Never use `f32` or `f64` seconds for timeline editing math, cuts, or audio sync. All time operations use 64-bit rational frames (`RationalTime { frames: i64, timebase: (u32, u32) }`).
2. **Unclipped 32-Bit Linear Color**: Every color transformation, CDL math, curve evaluation, and 3D LUT tetrahedral lookup runs in unbounded IEEE 754 32-bit floating-point space. Never clamp intermediate pixel data to $[0.0, 1.0]$ in internal nodes.

---

## ⚡ Part II: Vibecoding Rules (Velocity, Flow & High Aesthetics)

"Vibecoding" is the art of rapid, fluid, intuitive creation where the developer and AI iterate at lightning speed while maintaining uncompromising software craftsmanship and aesthetic excellence.

### 1. Build Working Features, Not Empty Placeholders
- **No Stubs or Mockups**: Don't leave empty `TODO: implement later` functions that pretend to work. If you implement a widget (like a color wheel or a timeline ruler), build the actual interactive logic, coordinates, dragging mechanics, and visual feedback immediately.
- **Immediate Playability**: Ensure every new component can be rendered, clicked, scrubbed, or inspected right away.

### 2. A-Grade Studio Aesthetics (Wow Factor on First Frame)
- **Calibrated Dark Studio Elegance**: Follow the D65 neutral studio dark palette. Use subtle micro-contrast, elegant borders, clean typography, and tactile active/hover states.
- **Fluid Ergonomics**: Every dial, slider, and scrub handle must feel weighted, responsive, and tactile. Ensure immediate-mode redraws feel smooth at 60–144 FPS.

### 3. Maintain Creative Flow & Autonomous Velocity
- **Smart Defaults Over Interruption**: Never stall development or prompt the user for trivial, mechanical decisions. Choose the most idiomatic, professional default, document the rationale, and keep moving forward.
- **Fearless Refactoring**: When code begins to get tangled, refactor boldly into clean structs, modular traits, and separate files before technical debt accumulates.
- **Root-Cause Fixes**: Never apply duct-tape patches. If an architectural assumption is flawed, adapt the design cleanly and comprehensively.

### 4. Continuous Verification & Clean Workspace
- **Zero-Warning Discipline**: Keep the codebase clean of dead code, unused imports, and clippy warnings.
- **Atomic Iterations**: Keep iterations self-contained and verifiable so the codebase always builds and tests cleanly at any point in time.
