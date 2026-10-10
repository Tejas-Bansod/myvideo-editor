# Vibecoding Rules — StudioGrade NLE

## 1. Flow State & Autonomous Velocity
- **Smart Defaults**: Make decisive, professional engineering choices without interrupting the user for trivial decisions.
- **Root-Cause Resolution**: Fix issues thoroughly at their architectural origin instead of applying cosmetic surface patches.
- **Momentum**: Keep iterations tight, functional, and self-contained. Always leave the workspace in a compile-ready state.

## 2. No Stubs, Full Polish
- **Working Implementation First**: Avoid shallow `TODO` stubs or non-functional placeholder mockups. When implementing widgets or engine logic, build full, functional, interactable implementations immediately.
- **Immediate Playability**: Ensure every panel, dial, slider, and shortcut can be engaged and tested right away.

## 3. High-Fidelity Studio Aesthetics
- **Calibrated Neutral Grays**: Strictly adhere to the D65 studio neutral dark palette (`StudioColors`) to preserve color grading accuracy.
- **Tactile Micro-Interactions**: Custom `egui::Painter` widgets must provide responsive hover states, smooth drag sensitivity, crisp borders, and silky 60–144 FPS responsiveness.

## 4. Bold Refactoring & Architecture Cleanliness
- **Fearless Refactoring**: As complexity grows, cleanly decompose into dedicated modules, traits, and clean structs.
- **Zero Warnings**: Maintain clean compilation with zero compiler warnings and clean clippy lints.
