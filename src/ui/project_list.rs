use crate::models::ProjectItem;
use crate::theme::StudioColors;
use eframe::egui::{self, Color32, CornerRadius, Pos2, Rect, Stroke, Vec2};
use egui_phosphor::regular as ph;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProjectViewMode {
    Grid,
    List,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProjectFilterTab {
    Local,
    Cloud,
    Trash,
}

pub struct ProjectListView;

impl ProjectListView {
    pub fn render(
        ui: &mut egui::Ui,
        projects: &mut Vec<ProjectItem>,
        view_mode: &mut ProjectViewMode,
        filter_tab: &mut ProjectFilterTab,
        search_query: &str,
        on_open_project_id: &mut Option<String>,
    ) {
        let tab_project_count = projects
            .iter()
            .filter(|p| match *filter_tab {
                ProjectFilterTab::Local => true,
                ProjectFilterTab::Cloud => p.is_cloud_synced,
                ProjectFilterTab::Trash => false,
            })
            .count();

        ui.vertical(|ui| {
            // ── Header row ────────────────────────────────────────────────────
            ui.horizontal(|ui| {
                ui.label(
                    egui::RichText::new("Projects")
                        .size(17.0)
                        .strong()
                        .color(StudioColors::TEXT_PRIMARY),
                );
                ui.add_space(6.0);
                ui.label(
                    egui::RichText::new(format!("{}", tab_project_count))
                        .size(11.0)
                        .color(StudioColors::TEXT_MUTED),
                );

                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    // Filter tabs: Local / Cloud / Trash
                    let tabs: &[(ProjectFilterTab, &str, &str)] = &[
                        (ProjectFilterTab::Trash, ph::TRASH, "Trash"),
                        (ProjectFilterTab::Cloud, ph::CLOUD, "Cloud"),
                        (ProjectFilterTab::Local, ph::HARD_DRIVE, "Local"),
                    ];
                    for (tab, icon, label) in tabs {
                        let is_active = *filter_tab == *tab;
                        let text = egui::RichText::new(format!("{}  {}", icon, label))
                            .size(11.0)
                            .color(if is_active {
                                Color32::WHITE
                            } else {
                                StudioColors::TEXT_MUTED
                            });
                        let btn = egui::Button::new(text)
                            .fill(if is_active {
                                Color32::from_rgb(18, 76, 102)
                            } else {
                                Color32::TRANSPARENT
                            })
                            .stroke(if is_active {
                                Stroke::new(1.0, StudioColors::ACCENT_CYAN)
                            } else {
                                Stroke::NONE
                            })
                            .corner_radius(CornerRadius::same(5))
                            .min_size(Vec2::new(66.0, 24.0));
                        let resp = ui.add(btn);
                        if resp.hovered() {
                            ui.ctx().set_cursor_icon(egui::CursorIcon::PointingHand);
                        }
                        if resp.clicked() {
                            *filter_tab = *tab;
                        }
                        ui.add_space(2.0);
                    }

                    ui.add_space(8.0);

                    // View mode toggle: Grid / List
                    for (mode, icon) in [
                        (ProjectViewMode::List, ph::LIST),
                        (ProjectViewMode::Grid, ph::GRID_FOUR),
                    ] {
                        let is_active = *view_mode == mode;
                        let btn = egui::Button::new(
                            egui::RichText::new(icon)
                                .size(13.0)
                                .color(if is_active {
                                    StudioColors::ACCENT_CYAN
                                } else {
                                    StudioColors::TEXT_MUTED
                                }),
                        )
                        .fill(if is_active {
                            Color32::from_rgb(18, 76, 102)
                        } else {
                            Color32::TRANSPARENT
                        })
                        .stroke(Stroke::NONE)
                        .corner_radius(CornerRadius::same(4))
                        .min_size(Vec2::new(28.0, 24.0));
                        let resp = ui.add(btn);
                        if resp.hovered() {
                            ui.ctx().set_cursor_icon(egui::CursorIcon::PointingHand);
                        }
                        if resp.clicked() {
                            *view_mode = mode;
                        }
                    }
                });
            });

            ui.add_space(10.0);

            // ── Filtered list ─────────────────────────────────────────────────
            let filtered_projects: Vec<&ProjectItem> = projects
                .iter()
                .filter(|p| match *filter_tab {
                    ProjectFilterTab::Local => true,
                    ProjectFilterTab::Cloud => p.is_cloud_synced,
                    ProjectFilterTab::Trash => false,
                })
                .filter(|p| {
                    search_query.is_empty()
                        || p.title.to_lowercase().contains(&search_query.to_lowercase())
                })
                .collect();

            if filtered_projects.is_empty() {
                ui.add_space(30.0);
                ui.vertical_centered(|ui| {
                    ui.label(
                        egui::RichText::new(ph::MAGNIFYING_GLASS)
                            .size(32.0)
                            .color(StudioColors::TEXT_MUTED),
                    );
                    ui.add_space(6.0);
                    ui.label(
                        egui::RichText::new("No projects found")
                            .size(14.0)
                            .color(StudioColors::TEXT_SECONDARY),
                    );
                });
                return;
            }

            match *view_mode {
                ProjectViewMode::Grid => {
                    Self::render_grid_view(ui, &filtered_projects, on_open_project_id);
                }
                ProjectViewMode::List => {
                    Self::render_list_view(ui, &filtered_projects, on_open_project_id);
                }
            }
        });
    }

    fn render_grid_view(
        ui: &mut egui::Ui,
        projects: &[&ProjectItem],
        on_open_project_id: &mut Option<String>,
    ) {
        let available_w = ui.available_width();
        let spacing = 8.0;
        // Compact: fit many small tiles per row
        let cols = (available_w / 115.0).floor().clamp(4.0, 12.0) as usize;
        let card_w =
            ((available_w - spacing * (cols.saturating_sub(1) as f32)) / cols as f32).max(80.0);
        // Square thumbnail + label area below
        let thumb_h = card_w;
        let label_h = 34.0;
        let card_h = thumb_h + label_h;

        egui::Grid::new("projects_grid")
            .spacing(Vec2::new(spacing, spacing))
            .show(ui, |ui| {
                for (idx, proj) in projects.iter().enumerate() {
                    let (card_rect, resp) = ui.allocate_exact_size(
                        Vec2::new(card_w, card_h),
                        egui::Sense::click(),
                    );
                    let painter = ui.painter();

                    let is_hovered = resp.hovered();
                    if is_hovered {
                        ui.ctx().set_cursor_icon(egui::CursorIcon::PointingHand);
                    }

                    // ── Square thumbnail ─────────────────────────────────────
                    let thumb_rect =
                        Rect::from_min_size(card_rect.min, Vec2::new(card_w, thumb_h));

                    let (r, g, b) = proj.thumbnail_accent;
                    let thumb_color = Color32::from_rgb(r / 2, g / 2, b / 2);

                    painter.rect_filled(
                        thumb_rect,
                        CornerRadius::same(5),
                        thumb_color,
                    );

                    // Hover border glow
                    if is_hovered {
                        painter.rect_stroke(
                            thumb_rect,
                            CornerRadius::same(5),
                            Stroke::new(1.5, StudioColors::ACCENT_CYAN),
                            egui::StrokeKind::Outside,
                        );
                    }

                    // Bottom accent line
                    painter.rect_filled(
                        Rect::from_min_size(
                            Pos2::new(thumb_rect.left(), thumb_rect.bottom() - 2.0),
                            Vec2::new(thumb_rect.width(), 2.0),
                        ),
                        CornerRadius { nw: 0, ne: 0, sw: 5, se: 5 },
                        Color32::from_rgb(r, g, b),
                    );

                    // Duration pill (bottom-right)
                    let dur_text = &proj.duration_str;
                    let dur_w = (dur_text.len() as f32 * 5.5 + 8.0).min(card_w - 4.0);
                    let dur_rect = Rect::from_min_size(
                        Pos2::new(thumb_rect.right() - dur_w - 3.0, thumb_rect.bottom() - 14.0),
                        Vec2::new(dur_w, 11.0),
                    );
                    painter.rect_filled(
                        dur_rect,
                        CornerRadius::same(2),
                        Color32::from_black_alpha(190),
                    );
                    painter.text(
                        dur_rect.center(),
                        egui::Align2::CENTER_CENTER,
                        dur_text,
                        egui::FontId::proportional(7.5),
                        Color32::WHITE,
                    );

                    // Cloud badge (top-right, tiny)
                    if proj.is_cloud_synced {
                        painter.text(
                            Pos2::new(thumb_rect.right() - 4.0, thumb_rect.top() + 10.0),
                            egui::Align2::RIGHT_CENTER,
                            ph::CLOUD_CHECK,
                            egui::FontId::proportional(9.0),
                            StudioColors::ACCENT_EMERALD,
                        );
                    }

                    // Play overlay on hover
                    if is_hovered {
                        painter.circle_filled(
                            thumb_rect.center(),
                            14.0,
                            Color32::from_black_alpha(210),
                        );
                        painter.text(
                            thumb_rect.center(),
                            egui::Align2::CENTER_CENTER,
                            ph::PLAY,
                            egui::FontId::proportional(11.0),
                            StudioColors::ACCENT_CYAN,
                        );
                    }

                    // ── Label area ───────────────────────────────────────────
                    let label_y = thumb_rect.bottom() + 2.0;

                    // Title — truncated to fit width
                    let max_chars = (card_w / 5.8) as usize;
                    let title = if proj.title.chars().count() > max_chars {
                        format!(
                            "{}…",
                            proj.title.chars().take(max_chars.saturating_sub(1)).collect::<String>()
                        )
                    } else {
                        proj.title.clone()
                    };
                    painter.text(
                        Pos2::new(card_rect.left(), label_y + 9.0),
                        egui::Align2::LEFT_CENTER,
                        title,
                        egui::FontId::proportional(10.5),
                        if is_hovered {
                            StudioColors::TEXT_PRIMARY
                        } else {
                            StudioColors::TEXT_SECONDARY
                        },
                    );

                    // Sub-meta: size | duration
                    painter.text(
                        Pos2::new(card_rect.left(), label_y + 24.0),
                        egui::Align2::LEFT_CENTER,
                        format!("{} | {}", proj.size_str, proj.duration_str),
                        egui::FontId::proportional(8.5),
                        StudioColors::TEXT_MUTED,
                    );

                    if resp.clicked() {
                        *on_open_project_id = Some(proj.id.clone());
                    }

                    if (idx + 1) % cols == 0 {
                        ui.end_row();
                    }
                }
            });
    }

    fn render_list_view(
        ui: &mut egui::Ui,
        projects: &[&ProjectItem],
        on_open_project_id: &mut Option<String>,
    ) {
        let row_h = 40.0;
        let w = ui.available_width();

        // Table Header
        let (header_rect, _) = ui.allocate_exact_size(Vec2::new(w, 28.0), egui::Sense::hover());
        let painter = ui.painter();
        painter.rect_filled(header_rect, CornerRadius::same(5), StudioColors::BG_CARD);

        let headers: &[(&str, &str, f32)] = &[
            (ph::FILM_STRIP, "Project Name", 0.0),
            (ph::MONITOR, "Resolution", 0.44),
            (ph::CLOCK, "Duration", 0.57),
            (ph::HARD_DRIVE, "File Size", 0.69),
            (ph::CALENDAR_BLANK, "Modified", 0.81),
        ];
        for (icon, label, x_frac) in headers {
            let x = if *x_frac == 0.0 {
                header_rect.left() + 36.0
            } else {
                header_rect.left() + w * x_frac
            };
            painter.text(
                Pos2::new(x, header_rect.center().y),
                egui::Align2::LEFT_CENTER,
                format!("{}  {}", icon, label),
                egui::FontId::proportional(10.0),
                StudioColors::TEXT_MUTED,
            );
        }

        ui.add_space(4.0);

        for proj in projects {
            let (rect, resp) = ui.allocate_exact_size(Vec2::new(w, row_h), egui::Sense::click());
            let painter = ui.painter();

            let is_hovered = resp.hovered();
            if is_hovered {
                ui.ctx().set_cursor_icon(egui::CursorIcon::PointingHand);
                painter.rect_filled(rect, CornerRadius::same(5), StudioColors::BG_CARD_HOVER);
            }

            let (r, g, b) = proj.thumbnail_accent;
            let dot_center = Pos2::new(rect.left() + 16.0, rect.center().y);
            painter.circle_filled(
                dot_center,
                8.0,
                Color32::from_rgba_premultiplied(r / 4, g / 4, b / 4, 100),
            );
            painter.circle_filled(dot_center, 5.0, Color32::from_rgb(r, g, b));

            painter.text(
                Pos2::new(rect.left() + 30.0, rect.center().y),
                egui::Align2::LEFT_CENTER,
                &proj.title,
                egui::FontId::proportional(12.0),
                StudioColors::TEXT_PRIMARY,
            );

            let res_rect = Rect::from_center_size(
                Pos2::new(rect.left() + w * 0.44 + 20.0, rect.center().y),
                Vec2::new(40.0, 15.0),
            );
            painter.rect_filled(res_rect, CornerRadius::same(3), StudioColors::BADGE_4K);
            painter.text(
                res_rect.center(),
                egui::Align2::CENTER_CENTER,
                &proj.resolution_str,
                egui::FontId::proportional(9.0),
                Color32::WHITE,
            );

            painter.text(
                Pos2::new(rect.left() + w * 0.57, rect.center().y),
                egui::Align2::LEFT_CENTER,
                &proj.duration_str,
                egui::FontId::proportional(11.0),
                StudioColors::TEXT_SECONDARY,
            );
            painter.text(
                Pos2::new(rect.left() + w * 0.69, rect.center().y),
                egui::Align2::LEFT_CENTER,
                &proj.size_str,
                egui::FontId::proportional(11.0),
                StudioColors::TEXT_MUTED,
            );
            painter.text(
                Pos2::new(rect.left() + w * 0.81, rect.center().y),
                egui::Align2::LEFT_CENTER,
                &proj.last_modified,
                egui::FontId::proportional(11.0),
                StudioColors::TEXT_MUTED,
            );

            if resp.clicked() {
                *on_open_project_id = Some(proj.id.clone());
            }
        }
    }
}
