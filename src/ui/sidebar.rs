use crate::theme::StudioColors;
use eframe::egui::{self, Color32, CornerRadius, Pos2, Rect, Stroke, StrokeKind, Vec2};
use egui_phosphor::regular as ph;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SidebarNav {
    Home,
    Projects,
    Templates,
    CloudSpaces,
    AiTools,
    Settings,
}

pub struct SidebarView;

impl SidebarView {
    pub fn render(ui: &mut egui::Ui, current_nav: &mut SidebarNav, on_create_click: &mut bool) {
        ui.vertical(|ui| {
            Self::render_profile_card(ui);
            ui.add_space(20.0);

            // New Project CTA Button with phosphor PLUS icon
            let btn_label = egui::RichText::new(format!("{}  New Project", ph::PLUS))
                .strong()
                .color(Color32::WHITE)
                .size(13.0);
            let create_btn = egui::Button::new(btn_label)
                .fill(StudioColors::ACCENT_BLUE)
                .stroke(Stroke::NONE)
                .corner_radius(CornerRadius::same(8))
                .min_size(Vec2::new(ui.available_width(), 36.0));

            let resp = ui.add(create_btn);
            if resp.hovered() {
                ui.ctx().set_cursor_icon(egui::CursorIcon::PointingHand);
            }
            if resp.clicked() {
                *on_create_click = true;
            }

            ui.add_space(20.0);

            ui.label(
                egui::RichText::new("WORKSPACE")
                    .size(9.0)
                    .color(StudioColors::TEXT_MUTED)
                    .strong(),
            );
            ui.add_space(4.0);

            // Nav items using Phosphor icons
            let nav_items: &[(SidebarNav, &str, &str)] = &[
                (SidebarNav::Home, ph::HOUSE, "Home"),
                (SidebarNav::Projects, ph::FOLDER_OPEN, "All Projects"),
                (SidebarNav::Templates, ph::FILM_SLATE, "Templates"),
                (SidebarNav::CloudSpaces, ph::CLOUD, "Cloud Spaces"),
                (SidebarNav::AiTools, ph::SPARKLE, "AI Creative Suite"),
            ];

            for (nav, icon, label) in nav_items {
                let is_active = *current_nav == *nav;
                let item_color = if is_active {
                    StudioColors::TEXT_PRIMARY
                } else {
                    StudioColors::TEXT_SECONDARY
                };

                let text = egui::RichText::new(format!("{}  {}", icon, label))
                    .size(12.5)
                    .color(item_color);

                let fill = if is_active {
                    StudioColors::BG_CARD_ACTIVE
                } else {
                    Color32::TRANSPARENT
                };

                let stroke = if is_active {
                    Stroke::new(1.0, StudioColors::ACCENT_CYAN)
                } else {
                    Stroke::NONE
                };

                let btn = egui::Button::new(text)
                    .fill(fill)
                    .stroke(stroke)
                    .corner_radius(CornerRadius::same(7))
                    .min_size(Vec2::new(ui.available_width(), 33.0));

                let resp = ui.add(btn);
                if resp.hovered() {
                    ui.ctx().set_cursor_icon(egui::CursorIcon::PointingHand);
                }
                if resp.clicked() {
                    *current_nav = *nav;
                }
            }

            let remaining_space = (ui.available_height() - 118.0).max(0.0);
            ui.add_space(remaining_space);
            Self::render_storage_widget(ui);
            ui.add_space(10.0);

            // Settings/Preferences row
            let settings_active = *current_nav == SidebarNav::Settings;
            let text = egui::RichText::new(format!("{}  Preferences", ph::GEAR_SIX))
                .size(12.5)
                .color(if settings_active {
                    StudioColors::TEXT_PRIMARY
                } else {
                    StudioColors::TEXT_SECONDARY
                });
            let resp = ui.add(
                egui::Button::new(text)
                    .fill(if settings_active {
                        StudioColors::BG_CARD_ACTIVE
                    } else {
                        Color32::TRANSPARENT
                    })
                    .corner_radius(CornerRadius::same(7))
                    .min_size(Vec2::new(ui.available_width(), 32.0)),
            );
            if resp.hovered() {
                ui.ctx().set_cursor_icon(egui::CursorIcon::PointingHand);
            }
            if resp.clicked() {
                *current_nav = SidebarNav::Settings;
            }
        });
    }

    fn render_profile_card(ui: &mut egui::Ui) {
        let (rect, _) =
            ui.allocate_exact_size(Vec2::new(ui.available_width(), 56.0), egui::Sense::hover());
        let painter = ui.painter();

        painter.rect_filled(rect, CornerRadius::same(8), StudioColors::BG_CARD);
        painter.rect_stroke(
            rect,
            CornerRadius::same(8),
            Stroke::new(1.0, StudioColors::BORDER_SUBTLE),
            StrokeKind::Inside,
        );

        // Avatar circle with gradient-style background
        let avatar_center = Pos2::new(rect.left() + 26.0, rect.center().y);
        // Outer glow ring
        painter.circle_stroke(
            avatar_center,
            17.5,
            Stroke::new(1.5, Color32::from_rgba_premultiplied(34, 211, 238, 60)),
        );
        painter.circle_filled(avatar_center, 16.0, StudioColors::ACCENT_CYAN);
        painter.text(
            avatar_center,
            egui::Align2::CENTER_CENTER,
            "S",
            egui::FontId::proportional(14.5),
            Color32::from_rgb(8, 18, 26),
        );

        // Name & Tier
        let text_start_x = rect.left() + 50.0;
        painter.text(
            Pos2::new(text_start_x, rect.center().y - 9.0),
            egui::Align2::LEFT_CENTER,
            "Studio Creator",
            egui::FontId::proportional(11.5),
            StudioColors::TEXT_PRIMARY,
        );

        // Pro Badge with brighter purple
        let badge_rect = Rect::from_min_size(
            Pos2::new(text_start_x, rect.center().y + 3.0),
            Vec2::new(36.0, 13.0),
        );
        painter.rect_filled(
            badge_rect,
            CornerRadius::same(4),
            StudioColors::ACCENT_PURPLE,
        );
        painter.text(
            badge_rect.center(),
            egui::Align2::CENTER_CENTER,
            "PRO",
            egui::FontId::proportional(8.5),
            Color32::WHITE,
        );
    }

    fn render_storage_widget(ui: &mut egui::Ui) {
        ui.vertical(|ui| {
            // Card container
            let (bg_rect, _) = ui.allocate_exact_size(
                Vec2::new(ui.available_width(), 44.0),
                egui::Sense::hover(),
            );
            let painter = ui.painter();
            painter.rect_filled(bg_rect, CornerRadius::same(6), StudioColors::BG_CARD);
            painter.rect_stroke(
                bg_rect,
                CornerRadius::same(6),
                Stroke::new(1.0, StudioColors::BORDER_SUBTLE),
                StrokeKind::Inside,
            );

            // Cloud icon + label
            painter.text(
                Pos2::new(bg_rect.left() + 10.0, bg_rect.top() + 14.0),
                egui::Align2::LEFT_CENTER,
                ph::HARD_DRIVE,
                egui::FontId::proportional(13.0),
                StudioColors::ACCENT_CYAN,
            );
            painter.text(
                Pos2::new(bg_rect.left() + 26.0, bg_rect.top() + 14.0),
                egui::Align2::LEFT_CENTER,
                "Cloud Storage",
                egui::FontId::proportional(10.5),
                StudioColors::TEXT_MUTED,
            );
            painter.text(
                Pos2::new(bg_rect.right() - 8.0, bg_rect.top() + 14.0),
                egui::Align2::RIGHT_CENTER,
                "42.5 / 250 GB",
                egui::FontId::proportional(10.0),
                StudioColors::TEXT_SECONDARY,
            );

            // Progress bar track
            let bar_left = bg_rect.left() + 10.0;
            let bar_right = bg_rect.right() - 10.0;
            let bar_w = bar_right - bar_left;
            let bar_y = bg_rect.top() + 32.0;
            let bar_rect = Rect::from_min_size(
                Pos2::new(bar_left, bar_y),
                Vec2::new(bar_w, 5.0),
            );
            painter.rect_filled(bar_rect, CornerRadius::same(3), StudioColors::BG_ELEVATED);

            // Progress fill (17%)
            let filled_w = (bar_w * 0.17).max(8.0);
            let filled_rect = Rect::from_min_size(bar_rect.min, Vec2::new(filled_w, 5.0));
            painter.rect_filled(
                filled_rect,
                CornerRadius::same(3),
                StudioColors::ACCENT_CYAN,
            );
        });
    }
}
