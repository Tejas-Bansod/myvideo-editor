use crate::models::AspectRatio;
use crate::theme::StudioColors;
use eframe::egui::{self, Color32, CornerRadius, Pos2, Rect, Stroke, Vec2};
use egui_phosphor::regular as ph;

pub struct HeroBannerView;

impl HeroBannerView {
    pub fn render(ui: &mut egui::Ui, selected_ratio: AspectRatio, on_create_project: &mut bool) {
        let available_w = ui.available_width();
        let hero_h = 140.0;

        let primary_w = (available_w * 0.66).max(360.0);
        let featured_w = (available_w - primary_w - 16.0).max(180.0);

        ui.horizontal(|ui| {
            // ── 1. PRIMARY "Create Project" Hero Card ──────────────────────────
            let (rect, response) =
                ui.allocate_exact_size(Vec2::new(primary_w, hero_h), egui::Sense::click());
            let painter = ui.painter();

            let is_hovered = response.hovered();
            if is_hovered {
                ui.ctx().set_cursor_icon(egui::CursorIcon::PointingHand);
            }

            // Gradient — cyan → oceanic blue
            let base_cyan = if is_hovered {
                Color32::from_rgb(48, 220, 250)
            } else {
                Color32::from_rgb(34, 195, 230)
            };
            let base_blue = if is_hovered {
                Color32::from_rgb(32, 118, 228)
            } else {
                Color32::from_rgb(25, 95, 200)
            };

            let steps = 32;
            let step_w = rect.width() / steps as f32;
            for i in 0..steps {
                let t = i as f32 / steps as f32;
                let r = (base_cyan.r() as f32 * (1.0 - t) + base_blue.r() as f32 * t) as u8;
                let g = (base_cyan.g() as f32 * (1.0 - t) + base_blue.g() as f32 * t) as u8;
                let b = (base_cyan.b() as f32 * (1.0 - t) + base_blue.b() as f32 * t) as u8;
                let slice_rect = Rect::from_min_size(
                    Pos2::new(rect.left() + i as f32 * step_w, rect.top()),
                    Vec2::new(step_w + 1.0, rect.height()),
                );
                painter.rect_filled(slice_rect, CornerRadius::ZERO, Color32::from_rgb(r, g, b));
            }

            // Border + rounding clip
            let border_stroke = if is_hovered {
                Stroke::new(2.0, Color32::WHITE)
            } else {
                Stroke::new(1.0, Color32::from_white_alpha(90))
            };
            painter.rect_stroke(
                rect,
                CornerRadius::same(12),
                border_stroke,
                egui::StrokeKind::Inside,
            );

            // Large Phosphor PLUS_CIRCLE icon in a frosted disc
            let icon_center = Pos2::new(rect.left() + 50.0, rect.center().y - 4.0);
            painter.circle_filled(icon_center, 26.0, Color32::from_black_alpha(55));
            painter.circle_stroke(
                icon_center,
                26.0,
                Stroke::new(1.5, Color32::from_white_alpha(140)),
            );
            painter.text(
                icon_center,
                egui::Align2::CENTER_CENTER,
                ph::PLUS_CIRCLE,
                egui::FontId::proportional(24.0),
                Color32::WHITE,
            );

            // Title & sub-description
            let text_x = rect.left() + 92.0;
            painter.text(
                Pos2::new(text_x, rect.center().y - 20.0),
                egui::Align2::LEFT_CENTER,
                "New Project",
                egui::FontId::proportional(23.0),
                Color32::WHITE,
            );
            painter.text(
                Pos2::new(text_x, rect.center().y + 6.0),
                egui::Align2::LEFT_CENTER,
                "Start a new studio timeline with 32-bit floating-point color science",
                egui::FontId::proportional(12.0),
                Color32::from_white_alpha(215),
            );

            // Aspect ratio pill
            let (def_w, def_h) = selected_ratio.default_resolution();
            let ratio_label = match selected_ratio {
                AspectRatio::Widescreen16x9 => "16:9",
                AspectRatio::Vertical9x16 => "9:16",
                AspectRatio::Square1x1 => "1:1",
                AspectRatio::Portrait4x5 => "4:5",
                AspectRatio::Ultrawide21x9 => "21:9",
            };
            let pill_text = format!(
                "{}  {} • {}×{}",
                ph::CROP,
                ratio_label,
                def_w,
                def_h
            );
            let pill_rect = Rect::from_min_size(
                Pos2::new(text_x, rect.center().y + 30.0),
                Vec2::new(160.0, 20.0),
            );
            painter.rect_filled(
                pill_rect,
                CornerRadius::same(5),
                Color32::from_black_alpha(75),
            );
            painter.text(
                Pos2::new(pill_rect.left() + 8.0, pill_rect.center().y),
                egui::Align2::LEFT_CENTER,
                pill_text,
                egui::FontId::proportional(10.5),
                Color32::WHITE,
            );

            // Keyboard hint (bottom-right corner)
            painter.text(
                Pos2::new(rect.right() - 14.0, rect.bottom() - 14.0),
                egui::Align2::RIGHT_CENTER,
                "⌘ N",
                egui::FontId::proportional(10.0),
                Color32::from_white_alpha(120),
            );

            if response.clicked() {
                *on_create_project = true;
            }

            ui.add_space(14.0);

            // ── 2. Featured AI/Tool Card (right) ─────────────────────────────
            let (f_rect, f_resp) =
                ui.allocate_exact_size(Vec2::new(featured_w, hero_h), egui::Sense::click());
            let f_painter = ui.painter();
            if f_resp.hovered() {
                ui.ctx().set_cursor_icon(egui::CursorIcon::PointingHand);
            }

            let f_bg = if f_resp.hovered() {
                StudioColors::BG_CARD_HOVER
            } else {
                StudioColors::BG_CARD
            };
            f_painter.rect_filled(f_rect, CornerRadius::same(12), f_bg);
            f_painter.rect_stroke(
                f_rect,
                CornerRadius::same(12),
                Stroke::new(
                    if f_resp.hovered() { 1.5 } else { 1.0 },
                    if f_resp.hovered() {
                        StudioColors::ACCENT_PURPLE
                    } else {
                        StudioColors::BORDER_SUBTLE
                    },
                ),
                egui::StrokeKind::Inside,
            );

            // Sparkle icon in purple disc
            let feature_icon_center = Pos2::new(f_rect.left() + 28.0, f_rect.top() + 34.0);
            f_painter.circle_filled(
                feature_icon_center,
                17.0,
                Color32::from_rgba_premultiplied(139, 92, 246, 40),
            );
            f_painter.text(
                feature_icon_center,
                egui::Align2::CENTER_CENTER,
                ph::SPARKLE,
                egui::FontId::proportional(18.0),
                StudioColors::ACCENT_PURPLE,
            );

            // "NEW" badge
            let badge_rect = Rect::from_min_size(
                Pos2::new(f_rect.left() + 50.0, f_rect.top() + 22.0),
                Vec2::new(36.0, 15.0),
            );
            f_painter.rect_filled(
                badge_rect,
                CornerRadius::same(4),
                StudioColors::ACCENT_PURPLE,
            );
            f_painter.text(
                badge_rect.center(),
                egui::Align2::CENTER_CENTER,
                "NEW",
                egui::FontId::proportional(9.0),
                Color32::WHITE,
            );

            f_painter.text(
                Pos2::new(f_rect.left() + 14.0, f_rect.top() + 58.0),
                egui::Align2::LEFT_CENTER,
                "AI Auto-Color Match",
                egui::FontId::proportional(14.0),
                StudioColors::TEXT_PRIMARY,
            );
            f_painter.text(
                Pos2::new(f_rect.left() + 14.0, f_rect.top() + 80.0),
                egui::Align2::LEFT_CENTER,
                "Match grades from reference",
                egui::FontId::proportional(11.0),
                StudioColors::TEXT_MUTED,
            );
            f_painter.text(
                Pos2::new(f_rect.left() + 14.0, f_rect.top() + 96.0),
                egui::Align2::LEFT_CENTER,
                "frames across multi-cam clips",
                egui::FontId::proportional(11.0),
                StudioColors::TEXT_MUTED,
            );

            // "Explore →" CTA with Phosphor ARROW_RIGHT
            let cta_text = format!("Explore Feature  {}", ph::ARROW_RIGHT);
            f_painter.text(
                Pos2::new(f_rect.left() + 14.0, f_rect.bottom() - 16.0),
                egui::Align2::LEFT_CENTER,
                cta_text,
                egui::FontId::proportional(11.5),
                StudioColors::ACCENT_CYAN,
            );
        });
    }
}
