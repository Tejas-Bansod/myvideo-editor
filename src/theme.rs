use eframe::egui::{self, Color32, CornerRadius, Stroke, Vec2, Visuals};

pub struct StudioColors;

impl StudioColors {
    // Deep studio neutral background tones (D65 compliant)
    pub const BG_APP: Color32 = Color32::from_rgb(15, 17, 21);
    pub const BG_SIDEBAR: Color32 = Color32::from_rgb(13, 14, 17);
    pub const BG_CARD: Color32 = Color32::from_rgb(21, 23, 29);
    pub const BG_ELEVATED: Color32 = Color32::from_rgb(25, 28, 35);
    pub const BG_CARD_HOVER: Color32 = Color32::from_rgb(31, 35, 44);
    pub const BG_CARD_ACTIVE: Color32 = Color32::from_rgb(37, 42, 52);

    // Subtle borders & dividers
    pub const BORDER_SUBTLE: Color32 = Color32::from_rgb(42, 46, 57);
    pub const BORDER_FOCUS: Color32 = Color32::from_rgb(73, 82, 101);

    // Primary Text & Hierarchy
    pub const TEXT_PRIMARY: Color32 = Color32::from_rgb(240, 243, 248);
    pub const TEXT_SECONDARY: Color32 = Color32::from_rgb(160, 166, 178);
    pub const TEXT_MUTED: Color32 = Color32::from_rgb(110, 116, 128);

    // Vibrant Hero Gradients & Accents
    pub const ACCENT_CYAN: Color32 = Color32::from_rgb(34, 211, 238);
    pub const ACCENT_BLUE: Color32 = Color32::from_rgb(59, 130, 246);
    pub const ACCENT_PURPLE: Color32 = Color32::from_rgb(139, 92, 246);
    pub const ACCENT_EMERALD: Color32 = Color32::from_rgb(16, 185, 129);
    pub const ACCENT_AMBER: Color32 = Color32::from_rgb(251, 191, 36);

    // Project badge colors
    pub const BADGE_4K: Color32 = Color32::from_rgb(244, 63, 94);
}

pub fn apply_studio_theme(ctx: &egui::Context) {
    let mut visuals = Visuals::dark();

    visuals.panel_fill = StudioColors::BG_SIDEBAR;
    visuals.window_fill = StudioColors::BG_CARD;
    visuals.extreme_bg_color = StudioColors::BG_APP;

    // Non-interactive widgets
    visuals.widgets.noninteractive.bg_fill = StudioColors::BG_CARD;
    visuals.widgets.noninteractive.bg_stroke = Stroke::new(1.0, StudioColors::BORDER_SUBTLE);
    visuals.widgets.noninteractive.fg_stroke = Stroke::new(1.0, StudioColors::TEXT_PRIMARY);
    visuals.widgets.noninteractive.corner_radius = CornerRadius::same(6);

    // Interactive inactive
    visuals.widgets.inactive.bg_fill = StudioColors::BG_CARD;
    visuals.widgets.inactive.bg_stroke = Stroke::new(1.0, StudioColors::BORDER_SUBTLE);
    visuals.widgets.inactive.fg_stroke = Stroke::new(1.0, StudioColors::TEXT_SECONDARY);
    visuals.widgets.inactive.corner_radius = CornerRadius::same(6);

    // Interactive hover
    visuals.widgets.hovered.bg_fill = StudioColors::BG_CARD_HOVER;
    visuals.widgets.hovered.bg_stroke = Stroke::new(1.0, StudioColors::BORDER_FOCUS);
    visuals.widgets.hovered.fg_stroke = Stroke::new(1.0, StudioColors::TEXT_PRIMARY);
    visuals.widgets.hovered.corner_radius = CornerRadius::same(6);

    // Interactive active / clicked
    visuals.widgets.active.bg_fill = StudioColors::BG_CARD_ACTIVE;
    visuals.widgets.active.bg_stroke = Stroke::new(1.5, StudioColors::ACCENT_CYAN);
    visuals.widgets.active.fg_stroke = Stroke::new(1.0, Color32::WHITE);
    visuals.widgets.active.corner_radius = CornerRadius::same(6);

    let mut style = egui::Style::default();
    style.visuals = visuals;
    style.spacing.item_spacing = Vec2::new(10.0, 8.0);
    style.spacing.button_padding = Vec2::new(12.0, 7.0);
    ctx.set_style_of(egui::Theme::Dark, style);
}
