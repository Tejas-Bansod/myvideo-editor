use crate::theme::StudioColors;
use eframe::egui::{self, Color32, CornerRadius, Pos2, Stroke, Vec2};
use egui_phosphor::regular as ph;

pub struct ToolStripView;

impl ToolStripView {
    pub fn render(ui: &mut egui::Ui) {
        ui.vertical(|ui| {
            ui.horizontal(|ui| {
                ui.label(
                    egui::RichText::new("Creative Tools Hub")
                        .size(13.0)
                        .strong()
                        .color(StudioColors::TEXT_PRIMARY),
                );

                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    let btn = egui::Button::new(
                        egui::RichText::new(format!("All Tools (14)  {}", ph::ARROW_RIGHT))
                            .size(11.0)
                            .color(StudioColors::ACCENT_CYAN),
                    )
                    .fill(Color32::TRANSPARENT)
                    .stroke(Stroke::NONE);
                    let resp = ui.add(btn);
                    if resp.hovered() {
                        ui.ctx().set_cursor_icon(egui::CursorIcon::PointingHand);
                    }
                });
            });

            ui.add_space(8.0);

            // Tool entries: (phosphor_icon, name, tag, accent_color)
            let tools: &[(&str, &str, &str, Color32)] = &[
                (
                    ph::DEVICE_MOBILE_CAMERA,
                    "Long to Shorts",
                    "Smart viral crop",
                    StudioColors::ACCENT_CYAN,
                ),
                (
                    ph::TRANSLATE,
                    "Video Translator",
                    "Multi-lingual dubbing",
                    StudioColors::ACCENT_BLUE,
                ),
                (
                    ph::MICROPHONE_STAGE,
                    "Text to Speech",
                    "Neural studio voices",
                    StudioColors::ACCENT_PURPLE,
                ),
                (
                    ph::PAINT_BUCKET,
                    "LUT Generator",
                    "ACES 3D .cube looks",
                    StudioColors::ACCENT_EMERALD,
                ),
                (
                    ph::WAVEFORM_SLASH,
                    "Voice Isolator",
                    "Clean background noise",
                    StudioColors::ACCENT_AMBER,
                ),
                (
                    ph::CROP,
                    "Auto Reframe",
                    "Dynamic subject tracker",
                    StudioColors::ACCENT_CYAN,
                ),
                (
                    ph::CHART_LINE_UP,
                    "Speed Curves",
                    "Velocity ramp presets",
                    StudioColors::ACCENT_BLUE,
                ),
            ];

            let item_w = 112.0;
            let item_h = 80.0;

            egui::ScrollArea::horizontal()
                .auto_shrink([false, true])
                .show(ui, |ui| {
                    ui.horizontal(|ui| {
                        for (icon, name, tag, accent) in tools {
                            let (rect, resp) = ui.allocate_exact_size(
                                Vec2::new(item_w, item_h),
                                egui::Sense::click(),
                            );
                            let painter = ui.painter();

                            let is_hovered = resp.hovered();
                            if is_hovered {
                                ui.ctx().set_cursor_icon(egui::CursorIcon::PointingHand);
                            }

                            let bg_color = if is_hovered {
                                StudioColors::BG_CARD_HOVER
                            } else {
                                StudioColors::BG_CARD
                            };

                            painter.rect_filled(rect, CornerRadius::same(10), bg_color);
                            painter.rect_stroke(
                                rect,
                                CornerRadius::same(10),
                                Stroke::new(
                                    if is_hovered { 1.5 } else { 1.0 },
                                    if is_hovered {
                                        *accent
                                    } else {
                                        StudioColors::BORDER_SUBTLE
                                    },
                                ),
                                egui::StrokeKind::Inside,
                            );

                            // Colored icon disc
                            let icon_center = Pos2::new(rect.center().x, rect.top() + 26.0);
                            painter.circle_filled(
                                icon_center,
                                16.0,
                                Color32::from_rgba_premultiplied(
                                    accent.r() / 5,
                                    accent.g() / 5,
                                    accent.b() / 5,
                                    220,
                                ),
                            );
                            painter.circle_stroke(
                                icon_center,
                                16.0,
                                Stroke::new(
                                    1.0,
                                    Color32::from_rgba_premultiplied(
                                        accent.r(),
                                        accent.g(),
                                        accent.b(),
                                        60,
                                    ),
                                ),
                            );
                            painter.text(
                                icon_center,
                                egui::Align2::CENTER_CENTER,
                                icon,
                                egui::FontId::proportional(15.0),
                                *accent,
                            );

                            // Tool name
                            painter.text(
                                Pos2::new(rect.center().x, rect.top() + 51.0),
                                egui::Align2::CENTER_CENTER,
                                name,
                                egui::FontId::proportional(10.5),
                                StudioColors::TEXT_PRIMARY,
                            );

                            // Subtle tag below
                            painter.text(
                                Pos2::new(rect.center().x, rect.top() + 65.0),
                                egui::Align2::CENTER_CENTER,
                                tag,
                                egui::FontId::proportional(8.5),
                                StudioColors::TEXT_MUTED,
                            );

                            if resp.clicked() {}
                        }
                    });
                });
        });
    }
}
