# 05. Frontend UI/UX Design System

## 1. Design System Philosophy & Studio Theme

StudioGrade NLE is engineered with a distraction-free, professional **dark studio interface** optimized for extended editing sessions and color grading accuracy. Built entirely in **Rust** using **`eframe` 0.36** and **`egui` 0.36**, the client avoids the memory bloat and garbage collection pauses of web-wrapper frameworks.

The visual system uses neutral dark gray tones calibrated against standard studio grading environment requirements (CIE D65 surround reflection). User interface elements maintain strictly neutral chromaticity so they never bias an editor's perception of color, hue, or white balance.

```
┌────────────────────────────────────────────────────────────────────────┐
│ Header: Project Title • Timecode [01:24:12:08] • Tabs • Export Action  │
├───────────────────┬───────────────────────────────┬────────────────────┤
│ Media Pool & Assets│ Native wgpu Preview Surface   │ Inspector Panel    │
│ • Video Footage   │ [Zero-Copy GPU Texture]       │ • Transform (X,Y,S)│
│ • Audio Tracks    │ Transport Controls (J-K-L)    │ • Audio Mixer / EQ │
│ • Titles & FX     │                               │ • Color Node Props │
├───────────────────┴───────────────────────────────┴────────────────────┤
│ Timeline Panel / Precision Color Room                                  │
│ [Track Headers] ─── Multi-Track Video & Audio Clip Lanes ──────────── │
│ [Color Wheels: Lift • Gamma • Gain • Offset] [Real-Time GPU Scopes]    │
└────────────────────────────────────────────────────────────────────────┘
```

---

## 2. Studio Theme Configuration (`egui::Visuals` & `egui::Style`)

The desktop UI configures an immediate-mode `egui` dark style with customized neutral palette tokens:

```rust
// apps/desktop/src/theme.rs
use eframe::egui::{self, Color32, Rounding, Stroke, Vec2, Visuals};

pub struct StudioColors;

impl StudioColors {
    pub const BG_CANVAS: Color32 = Color32::from_rgb(18, 18, 20);
    pub const BG_PANEL: Color32 = Color32::from_rgb(26, 26, 29);
    pub const BG_WIDGET: Color32 = Color32::from_rgb(34, 34, 38);
    pub const BG_HOVER: Color32 = Color32::from_rgb(46, 46, 52);
    pub const BG_ACTIVE: Color32 = Color32::from_rgb(58, 58, 66);

    pub const BORDER: Color32 = Color32::from_rgb(45, 45, 49);
    pub const BORDER_FOCUS: Color32 = Color32::from_rgb(90, 90, 98);

    pub const TEXT_PRIMARY: Color32 = Color32::from_rgb(240, 240, 245);
    pub const TEXT_MUTED: Color32 = Color32::from_rgb(150, 150, 158);
    pub const ACCENT: Color32 = Color32::from_rgb(220, 220, 225);

    // Track lane identifiers
    pub const VIDEO_TRACK: Color32 = Color32::from_rgb(38, 70, 83);
    pub const AUDIO_TRACK: Color32 = Color32::from_rgb(42, 157, 143);
    pub const TITLE_TRACK: Color32 = Color32::from_rgb(231, 111, 81);
}

pub fn apply_studio_theme(ctx: &egui::Context) {
    let mut visuals = Visuals::dark();

    // Dark studio background and window panels
    visuals.panel_fill = StudioColors::BG_PANEL;
    visuals.window_fill = StudioColors::BG_PANEL;
    visuals.extreme_bg_color = StudioColors::BG_CANVAS;

    // Non-interactive widgets
    visuals.widgets.noninteractive.bg_fill = StudioColors::BG_PANEL;
    visuals.widgets.noninteractive.bg_stroke = Stroke::new(1.0, StudioColors::BORDER);
    visuals.widgets.noninteractive.fg_stroke = Stroke::new(1.0, StudioColors::TEXT_PRIMARY);
    visuals.widgets.noninteractive.rounding = Rounding::same(4.0);

    // Interactive widgets (buttons, dials, inputs)
    visuals.widgets.inactive.bg_fill = StudioColors::BG_WIDGET;
    visuals.widgets.inactive.bg_stroke = Stroke::new(1.0, StudioColors::BORDER);
    visuals.widgets.inactive.fg_stroke = Stroke::new(1.0, StudioColors::TEXT_PRIMARY);
    visuals.widgets.inactive.rounding = Rounding::same(4.0);

    visuals.widgets.hovered.bg_fill = StudioColors::BG_HOVER;
    visuals.widgets.hovered.bg_stroke = Stroke::new(1.0, StudioColors::BORDER_FOCUS);
    visuals.widgets.hovered.fg_stroke = Stroke::new(1.0, Color32::WHITE);
    visuals.widgets.hovered.rounding = Rounding::same(4.0);

    visuals.widgets.active.bg_fill = StudioColors::BG_ACTIVE;
    visuals.widgets.active.bg_stroke = Stroke::new(1.5, StudioColors::ACCENT);
    visuals.widgets.active.rounding = Rounding::same(4.0);

    let mut style = egui::Style::default();
    style.visuals = visuals;
    style.spacing.item_spacing = Vec2::new(8.0, 6.0);
    style.spacing.button_padding = Vec2::new(10.0, 5.0);
    ctx.set_style(style);
}
```

---

## 3. Core Component & Custom Widget Implementation (Rust + egui)

### 3.1 Workspace Tab Switcher
Immediate-mode workspace navigation allows rapid switching between dedicated editing pages:

```rust
// apps/desktop/src/panels/header.rs
use eframe::egui::{self, Ui};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WorkspaceTab {
    Edit,
    Color,
    Audio,
    Effects,
    Deliver,
}

pub fn render_workspace_tabs(ui: &mut Ui, current: &mut WorkspaceTab) {
    ui.horizontal(|ui| {
        let tabs = [
            (WorkspaceTab::Edit, "🎬 Edit"),
            (WorkspaceTab::Color, "🎨 Color"),
            (WorkspaceTab::Audio, "🎵 Audio"),
            (WorkspaceTab::Effects, "✨ Effects"),
            (WorkspaceTab::Deliver, "🚀 Deliver"),
        ];

        for (tab, label) in tabs {
            let is_selected = *current == tab;
            let btn = egui::Button::new(label)
                .fill(if is_selected {
                    crate::theme::StudioColors::BG_ACTIVE
                } else {
                    crate::theme::StudioColors::BG_WIDGET
                })
                .selected(is_selected);

            if ui.add(btn).clicked() {
                *current = tab;
            }
        }
    });
}
```

---

### 3.2 Precision Color Wheel Dial Component
A custom interactive `egui` widget implementing a 2D chromaticity polar disc with draggable crosshair indicator puck and a master horizontal luminance slider:

```rust
// apps/desktop/src/widgets/color_wheel.rs
use eframe::egui::{self, Pos2, Rect, Sense, Stroke, Vec2, Widget};
use crate::theme::StudioColors;

#[derive(Debug, Clone)]
pub struct ColorWheelState {
    pub label: String,
    pub rgb: [f32; 3],  // Chromaticity offset [-1.0, 1.0]
    pub luma: f32,       // Master luminance offset [-1.0, 1.0]
}

impl ColorWheelState {
    pub fn new(label: impl Into<String>) -> Self {
        Self {
            label: label.into(),
            rgb: [0.0, 0.0, 0.0],
            luma: 0.0,
        }
    }

    pub fn reset(&mut self) {
        self.rgb = [0.0, 0.0, 0.0];
        self.luma = 0.0;
    }
}

pub struct ColorWheel<'a> {
    state: &'a mut ColorWheelState,
    radius: f32,
}

impl<'a> ColorWheel<'a> {
    pub fn new(state: &'a mut ColorWheelState) -> Self {
        Self { state, radius: 64.0 }
    }
}

impl<'a> Widget for ColorWheel<'a> {
    fn ui(self, ui: &mut egui::Ui) -> egui::Response {
        let diameter = self.radius * 2.0;
        let total_size = Vec2::new(diameter + 16.0, diameter + 60.0);
        let (rect, mut response) = ui.allocate_exact_size(total_size, Sense::click_and_drag());

        let painter = ui.painter();
        let wheel_center = Pos2::new(rect.center().x, rect.top() + self.radius + 20.0);

        // 1. Draw Title Header & Reset Button
        let header_rect = Rect::from_min_size(rect.min, Vec2::new(rect.width(), 20.0));
        ui.allocate_ui_at_rect(header_rect, |ui| {
            ui.horizontal(|ui| {
                ui.label(egui::RichText::new(&self.state.label).strong().size(12.0));
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    if ui.small_button("↺").clicked() {
                        self.state.reset();
                        response.mark_changed();
                    }
                });
            });
        });

        // 2. Draw Polar Background Circle & Graticules
        painter.circle_filled(wheel_center, self.radius, StudioColors::BG_CANVAS);
        painter.circle_stroke(wheel_center, self.radius, Stroke::new(1.0, StudioColors::BORDER));
        painter.circle_stroke(wheel_center, self.radius * 0.5, Stroke::new(0.5, StudioColors::BORDER));
        painter.line_segment(
            [Pos2::new(wheel_center.x - self.radius, wheel_center.y), Pos2::new(wheel_center.x + self.radius, wheel_center.y)],
            Stroke::new(0.5, StudioColors::BORDER),
        );
        painter.line_segment(
            [Pos2::new(wheel_center.x, wheel_center.y - self.radius), Pos2::new(wheel_center.x, wheel_center.y + self.radius)],
            Stroke::new(0.5, StudioColors::BORDER),
        );

        // 3. Handle Mouse Interaction (2D Puck Dragging)
        if response.dragged() {
            if let Some(mouse_pos) = response.interact_pointer_pos() {
                let delta = mouse_pos - wheel_center;
                let dist = delta.length();
                let clamped_delta = if dist > self.radius {
                    delta * (self.radius / dist)
                } else {
                    delta
                };

                // Map to [-1.0, 1.0] chromaticity space
                self.state.rgb[0] = (clamped_delta.x / self.radius).clamp(-1.0, 1.0);
                self.state.rgb[1] = (-clamped_delta.y / self.radius).clamp(-1.0, 1.0);
                response.mark_changed();
            }
        }

        // 4. Draw Draggable Puck Indicator
        let puck_pos = Pos2::new(
            wheel_center.x + self.state.rgb[0] * self.radius,
            wheel_center.y - self.state.rgb[1] * self.radius,
        );
        painter.circle_filled(puck_pos, 4.0, StudioColors::ACCENT);
        painter.circle_stroke(puck_pos, 5.0, Stroke::new(1.5, egui::Color32::WHITE));

        // 5. Draw Master Luminance Slider
        let slider_rect = Rect::from_min_size(
            Pos2::new(rect.min.x + 8.0, rect.bottom() - 26.0),
            Vec2::new(rect.width() - 16.0, 20.0),
        );
        ui.allocate_ui_at_rect(slider_rect, |ui| {
            let slider_res = ui.add(
                egui::Slider::new(&mut self.state.luma, -1.0..=1.0)
                    .show_value(true)
                    .text("Y")
            );
            if slider_res.changed() {
                response.mark_changed();
            }
        });

        response
    }
}
```

---

### 3.3 Custom Real-Time Broadcast Scopes Painters
Scopes are rendered via immediate-mode custom painters running over GPU texture readbacks:

- **RGB Parade**: Plots Red, Green, and Blue columns side by side across horizontal graticule lines ($0\ \text{IRE}$ to $100\ \text{IRE}$).
- **Vectorscope**: Circular chrominance distribution with calibrated skin-tone indicator vector ($I$-line at $123^\circ$) and $75\%$ broadcast saturation reference boxes (Red, Magenta, Blue, Cyan, Green, Yellow).

---

## 4. Inspector & Dockable Panel Architecture

The right side panel is laid out using `egui::SidePanel` and collapsible headers for parameter precision:

```rust
// apps/desktop/src/panels/inspector.rs
use eframe::egui::{self, CollapsingHeader, DragValue, Slider, Ui};

pub fn render_inspector(ui: &mut Ui, state: &mut crate::app::InspectorState) {
    egui::ScrollArea::vertical().show(ui, |ui| {
        // 1. Transform Section
        CollapsingHeader::new("📐 Video Transform")
            .default_open(true)
            .show(ui, |ui| {
                ui.horizontal(|ui| {
                    ui.label("Position X:");
                    ui.add(DragValue::new(&mut state.pos_x).speed(1.0));
                    ui.label("Y:");
                    ui.add(DragValue::new(&mut state.pos_y).speed(1.0));
                });

                ui.horizontal(|ui| {
                    ui.label("Scale:");
                    ui.add(Slider::new(&mut state.scale, 0.1..=4.0).step_by(0.01));
                });

                ui.horizontal(|ui| {
                    ui.label("Rotation:");
                    ui.add(DragValue::new(&mut state.rotation).suffix("°").speed(0.5));
                });

                ui.horizontal(|ui| {
                    ui.label("Opacity:");
                    ui.add(Slider::new(&mut state.opacity, 0.0..=1.0));
                });
            });

        ui.separator();

        // 2. Audio Channel Controls
        CollapsingHeader::new("🔊 Audio Channel")
            .default_open(true)
            .show(ui, |ui| {
                ui.horizontal(|ui| {
                    ui.label("Volume (dB):");
                    ui.add(Slider::new(&mut state.audio_volume_db, -60.0..=12.0).suffix(" dB"));
                });

                ui.horizontal(|ui| {
                    ui.label("Pan:");
                    ui.add(Slider::new(&mut state.audio_pan, -1.0..=1.0));
                });

                ui.checkbox(&mut state.voice_isolation, "Voice Isolation (ML)");
            });

        ui.separator();

        // 3. Speed & Time Warping
        CollapsingHeader::new("⚡ Speed & Warping")
            .default_open(false)
            .show(ui, |ui| {
                ui.horizontal(|ui| {
                    ui.label("Multiplier:");
                    ui.add(Slider::new(&mut state.speed, 0.1..=10.0).suffix("x"));
                });
                ui.checkbox(&mut state.reverse, "Reverse Playback");
                ui.checkbox(&mut state.optical_flow, "Optical Flow Interpolation");
            });
    });
}
```

---

## 5. Keyboard Navigation & Studio Shortcuts

Immediate-mode keyboard event polling eliminates latency and handles high-velocity workflow shortcuts:

```rust
// apps/desktop/src/input.rs
use eframe::egui::{self, Key, Modifiers};

pub fn handle_global_shortcuts(ctx: &egui::Context, app: &mut crate::app::StudioApp) {
    ctx.input(|i| {
        // Playhead Play / Pause (Space)
        if i.key_pressed(Key::Space) {
            app.toggle_playback();
        }

        // Razor Cut at Playhead (Ctrl+B or C)
        if (i.modifiers.command && i.key_pressed(Key::B)) || i.key_pressed(Key::C) {
            app.cut_active_clip_at_playhead();
        }

        // Ripple Trim Head (Q) and Tail (W)
        if i.key_pressed(Key::Q) {
            app.ripple_trim_head();
        }
        if i.key_pressed(Key::W) {
            app.ripple_trim_tail();
        }

        // J-K-L Shuttling
        if i.key_pressed(Key::J) {
            app.shuttle_reverse();
        }
        if i.key_pressed(Key::K) {
            app.shuttle_pause();
        }
        if i.key_pressed(Key::L) {
            app.shuttle_forward();
        }

        // Toggle Real-Time Scopes (Ctrl + Shift + S)
        if i.modifiers.command && i.modifiers.shift && i.key_pressed(Key::S) {
            app.toggle_scopes_overlay();
        }
    });
}
```

### Shortcuts Reference Matrix

| Action | Shortcut | Engine Behavior |
| :--- | :--- | :--- |
| **Play / Pause** | `Space` | Toggles rational clock playback loop. |
| **Cut Active Clip** | `Ctrl + B` / `C` | Splits clip intervals at exact playhead frame. |
| **Ripple Trim Head** | `Q` | Trims in-point to playhead and shifts trailing clips left. |
| **Ripple Trim Tail** | `W` | Trims out-point to playhead and shifts trailing clips left. |
| **J-K-L Shuttle** | `J` / `K` / `L` | Steps shuttle speeds: $-4\times, -2\times, -1\times, 0, 1\times, 2\times, 4\times$. |
| **Previous / Next Cut** | `Up` / `Down` | Playhead jumps directly to nearest edit point. |
| **Timeline Zoom** | `Ctrl + =` / `Ctrl + -` | Adjusts horizontal pixels-per-frame view scale. |
| **Toggle Scopes** | `Ctrl + Shift + S` | Toggles floating Parade/Vectorscope overlay. |
| **Reset Grade Node** | `Alt + Home` | Resets active color grade node CDL parameters. |
