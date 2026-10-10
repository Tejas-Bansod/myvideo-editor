use crate::theme::StudioColors;
use eframe::egui::{self, Color32, CornerRadius, Pos2, Rect, Stroke, Vec2};
use egui_phosphor::regular as ph;

pub struct QuickToolsView;

impl QuickToolsView {
    pub fn render(ui: &mut egui::Ui) {
        // Quick-access tool cards with Phosphor icons
        let tools: &[(&str, &str, &str, Color32)] = &[
            (
                ph::FILM_SCRIPT,
                "Script to Video",
                "Generate storyboard timeline from script",
                StudioColors::ACCENT_CYAN,
            ),
            (
                ph::MONITOR_PLAY,
                "Screen Recorder",
                "Direct 4K capture with mic & system audio",
                StudioColors::ACCENT_BLUE,
            ),
            (
                ph::SCISSORS,
                "Smart Cut & Trimmer",
                "Auto-silence removal and beat-grid sync",
                StudioColors::ACCENT_EMERALD,
            ),
            (
                ph::SUBTITLES,
                "Auto Subtitles",
                "Whisper STT with kinetic caption presets",
                StudioColors::ACCENT_AMBER,
            ),
        ];

        let available_w = ui.available_width();
        let card_count = tools.len() as f32;
        let spacing = 10.0;
        let card_w = ((available_w - spacing * (card_count - 1.0)) / card_count).max(140.0);
        let card_h = 80.0;

        ui.horizontal(|ui| {
            for (icon, title, desc, accent) in tools {
                let (rect, response) =
                    ui.allocate_exact_size(Vec2::new(card_w, card_h), egui::Sense::click());
                let painter = ui.painter();

                let is_hovered = response.hovered();
                if is_hovered {
                    ui.ctx().set_cursor_icon(egui::CursorIcon::PointingHand);
                }

                let bg_color = if is_hovered {
                    StudioColors::BG_CARD_HOVER
                } else {
                    StudioColors::BG_CARD
                };

                let stroke_color = if is_hovered {
                    *accent
                } else {
                    StudioColors::BORDER_SUBTLE
                };

                // Card background
                painter.rect_filled(rect, CornerRadius::same(10), bg_color);
                painter.rect_stroke(
                    rect,
                    CornerRadius::same(10),
                    Stroke::new(if is_hovered { 1.5 } else { 1.0 }, stroke_color),
                    egui::StrokeKind::Inside,
                );

                // Accent left-side bar
                if is_hovered {
                    let bar_rect = Rect::from_min_size(
                        rect.min,
                        Vec2::new(3.0, rect.height()),
                    );
                    painter.rect_filled(
                        bar_rect,
                        CornerRadius { nw: 10, ne: 0, sw: 10, se: 0 },
                        *accent,
                    );
                }

                // Icon badge with accent tint background
                let icon_rect = Rect::from_min_size(
                    Pos2::new(rect.left() + 12.0, rect.top() + 14.0),
                    Vec2::new(30.0, 30.0),
                );
                painter.rect_filled(
                    icon_rect,
                    CornerRadius::same(7),
                    Color32::from_rgba_premultiplied(
                        accent.r() / 5,
                        accent.g() / 5,
                        accent.b() / 5,
                        200,
                    ),
                );
                painter.text(
                    icon_rect.center(),
                    egui::Align2::CENTER_CENTER,
                    icon,
                    egui::FontId::proportional(16.0),
                    *accent,
                );

                // Title
                painter.text(
                    Pos2::new(rect.left() + 50.0, rect.top() + 21.0),
                    egui::Align2::LEFT_CENTER,
                    title,
                    egui::FontId::proportional(12.5),
                    StudioColors::TEXT_PRIMARY,
                );

                // Description (smaller, muted)
                painter.text(
                    Pos2::new(rect.left() + 12.0, rect.bottom() - 18.0),
                    egui::Align2::LEFT_CENTER,
                    desc,
                    egui::FontId::proportional(10.0),
                    StudioColors::TEXT_MUTED,
                );

                // Arrow caret on hover
                if is_hovered {
                    painter.text(
                        Pos2::new(rect.right() - 14.0, rect.top() + 21.0),
                        egui::Align2::RIGHT_CENTER,
                        ph::CARET_RIGHT,
                        egui::FontId::proportional(14.0),
                        *accent,
                    );
                }

                if response.clicked() {
                    // Quick tool action handler
                }
            }
        });
    }
}
