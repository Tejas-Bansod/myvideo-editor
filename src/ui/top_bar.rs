use crate::models::AspectRatio;
use crate::theme::StudioColors;
use eframe::egui::{self, Color32, CornerRadius, Stroke, Vec2};
use egui_phosphor::regular as ph;

pub struct TopBarView;

impl TopBarView {
    pub fn render(
        ui: &mut egui::Ui,
        selected_ratio: &mut AspectRatio,
        search_query: &mut String,
        on_open_project: &mut bool,
    ) {
        ui.horizontal(|ui| {
            // Aspect ratio label with icon
            ui.label(
                egui::RichText::new(format!("{}  Aspect ratio", ph::CROP))
                    .size(11.0)
                    .color(StudioColors::TEXT_SECONDARY),
            );
            ui.add_space(4.0);

            let ratios = [
                (AspectRatio::Widescreen16x9, "16:9", "Landscape (1920×1080)"),
                (AspectRatio::Vertical9x16, "9:16", "Vertical / Reels (1080×1920)"),
                (AspectRatio::Square1x1, "1:1", "Square (1080×1080)"),
                (AspectRatio::Ultrawide21x9, "21:9", "Cinema (2560×1080)"),
            ];

            for (ratio, label, tooltip) in ratios {
                let is_selected = *selected_ratio == ratio;
                let text = egui::RichText::new(label).size(11.0).color(if is_selected {
                    Color32::WHITE
                } else {
                    StudioColors::TEXT_SECONDARY
                });

                let fill = if is_selected {
                    Color32::from_rgb(18, 76, 102)
                } else {
                    StudioColors::BG_ELEVATED
                };

                let stroke = if is_selected {
                    Stroke::new(1.0, StudioColors::ACCENT_CYAN)
                } else {
                    Stroke::new(1.0, StudioColors::BORDER_SUBTLE)
                };

                let btn = egui::Button::new(text)
                    .fill(fill)
                    .stroke(stroke)
                    .corner_radius(egui::CornerRadius::same(6))
                    .min_size(Vec2::new(46.0, 27.0));

                let resp = ui.add(btn).on_hover_text(tooltip);
                if resp.hovered() {
                    ui.ctx().set_cursor_icon(egui::CursorIcon::PointingHand);
                }
                if resp.clicked() {
                    *selected_ratio = ratio;
                }
            }

            ui.add_space(14.0);

            // Open Project button with Phosphor folder-open icon
            let open_btn_text = egui::RichText::new(format!("{}  Open Project", ph::FOLDER_OPEN))
                .size(11.5)
                .color(StudioColors::TEXT_PRIMARY);

            let open_btn = egui::Button::new(open_btn_text)
                .fill(StudioColors::BG_ELEVATED)
                .stroke(Stroke::new(1.0, StudioColors::BORDER_SUBTLE))
                .corner_radius(egui::CornerRadius::same(6))
                .min_size(Vec2::new(130.0, 27.0));

            let resp = ui
                .add(open_btn)
                .on_hover_text("Open .vedit or media project file from disk");
            if resp.hovered() {
                ui.ctx().set_cursor_icon(egui::CursorIcon::PointingHand);
            }
            if resp.clicked() {
                *on_open_project = true;
            }

            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                // Synced indicator with cloud-check icon
                ui.add_space(4.0);
                ui.label(
                    egui::RichText::new(format!("{}  Synced", ph::CLOUD_CHECK))
                        .size(11.0)
                        .color(StudioColors::ACCENT_EMERALD),
                );

                ui.add_space(10.0);

                // Help button with question-mark icon
                let help_btn = egui::Button::new(
                    egui::RichText::new(format!("{}  Help", ph::QUESTION))
                        .size(11.0)
                        .color(StudioColors::TEXT_SECONDARY),
                )
                .fill(StudioColors::BG_ELEVATED)
                .stroke(Stroke::new(1.0, StudioColors::BORDER_SUBTLE))
                .corner_radius(CornerRadius::same(6))
                .min_size(Vec2::new(70.0, 27.0));

                if ui.add(help_btn).clicked() {}

                ui.add_space(10.0);

                // Search field with icon label prefix
                ui.horizontal(|ui| {
                    ui.label(
                        egui::RichText::new(ph::MAGNIFYING_GLASS)
                            .size(13.0)
                            .color(StudioColors::TEXT_MUTED),
                    );
                    let search_field = egui::TextEdit::singleline(search_query)
                        .hint_text("Search projects…")
                        .desired_width(160.0);
                    ui.add(search_field);
                });
            });
        });
    }
}
