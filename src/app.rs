use crate::models::{AspectRatio, ProjectItem};
use crate::theme::{apply_studio_theme, StudioColors};
use crate::ui::hero_banner::HeroBannerView;
use crate::ui::new_project_modal::{NewProjectConfig, NewProjectModal};
use crate::ui::project_list::{ProjectFilterTab, ProjectListView, ProjectViewMode};
use crate::ui::quick_tools::QuickToolsView;
use crate::ui::sidebar::{SidebarNav, SidebarView};
use crate::ui::tool_strip::ToolStripView;
use crate::ui::top_bar::TopBarView;
use eframe::egui::{self, Color32, CornerRadius, Stroke, Vec2};
use egui_phosphor::regular as ph;

pub struct LauncherApp {
    pub sidebar_nav: SidebarNav,
    pub selected_aspect_ratio: AspectRatio,
    pub search_query: String,
    pub projects: Vec<ProjectItem>,
    pub view_mode: ProjectViewMode,
    pub filter_tab: ProjectFilterTab,

    pub is_new_project_modal_open: bool,
    pub new_project_config: NewProjectConfig,

    pub opened_project_notice: Option<String>,
}

impl Default for LauncherApp {
    fn default() -> Self {
        Self {
            sidebar_nav: SidebarNav::Home,
            selected_aspect_ratio: AspectRatio::Widescreen16x9,
            search_query: String::new(),
            projects: ProjectItem::mock_projects(),
            view_mode: ProjectViewMode::Grid,
            filter_tab: ProjectFilterTab::Local,
            is_new_project_modal_open: false,
            new_project_config: NewProjectConfig::default(),
            opened_project_notice: None,
        }
    }
}

impl eframe::App for LauncherApp {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        apply_studio_theme(ui.ctx());

        let mut trigger_create_modal = false;
        let mut trigger_open_file_dialog = false;
        let mut selected_project_id: Option<String> = None;
        let mut newly_created_project: Option<ProjectItem> = None;

        let sidebar_width = 220.0;

        let app_rect = ui.max_rect();
        let sidebar_rect = egui::Rect::from_min_max(
            app_rect.min,
            egui::Pos2::new(app_rect.left() + sidebar_width, app_rect.bottom()),
        );
        let content_rect = egui::Rect::from_min_max(
            egui::Pos2::new(sidebar_rect.right() + 28.0, app_rect.top()),
            egui::Pos2::new(app_rect.right() - 32.0, app_rect.bottom()),
        );

        // Sidebar background
        ui.painter()
            .rect_filled(sidebar_rect, CornerRadius::ZERO, StudioColors::BG_SIDEBAR);
        ui.painter().rect_filled(
            egui::Rect::from_min_size(
                egui::Pos2::new(sidebar_rect.right(), sidebar_rect.top()),
                Vec2::new(1.0, sidebar_rect.height()),
            ),
            CornerRadius::ZERO,
            StudioColors::BORDER_SUBTLE,
        );

        // Sidebar
        ui.scope_builder(
            egui::UiBuilder::new()
                .id_salt("sidebar")
                .max_rect(sidebar_rect.shrink2(Vec2::new(12.0, 14.0)))
                .layout(egui::Layout::top_down(egui::Align::Min)),
            |ui| {
                SidebarView::render(ui, &mut self.sidebar_nav, &mut trigger_create_modal);
            },
        );

        // Main content area
        ui.scope_builder(
            egui::UiBuilder::new()
                .id_salt("project_workspace")
                .max_rect(content_rect)
                .layout(egui::Layout::top_down(egui::Align::Min)),
            |ui| {
                ui.add_space(28.0);

                // Top bar (aspect ratio + search + open)
                TopBarView::render(
                    ui,
                    &mut self.selected_aspect_ratio,
                    &mut self.search_query,
                    &mut trigger_open_file_dialog,
                );
                ui.add_space(22.0);

                // Project opened / created notice banner
                if let Some(notice) = &self.opened_project_notice {
                    let (notice_rect, _) = ui.allocate_exact_size(
                        Vec2::new(ui.available_width().min(560.0), 34.0),
                        egui::Sense::hover(),
                    );
                    let painter = ui.painter();
                    painter.rect_filled(
                        notice_rect,
                        CornerRadius::same(8),
                        Color32::from_rgb(14, 44, 34),
                    );
                    painter.rect_stroke(
                        notice_rect,
                        CornerRadius::same(8),
                        Stroke::new(1.0, StudioColors::ACCENT_EMERALD),
                        egui::StrokeKind::Inside,
                    );
                    // Check-circle icon
                    painter.text(
                        egui::pos2(notice_rect.left() + 12.0, notice_rect.center().y),
                        egui::Align2::LEFT_CENTER,
                        ph::CHECK_CIRCLE,
                        egui::FontId::proportional(14.0),
                        StudioColors::ACCENT_EMERALD,
                    );
                    painter.text(
                        egui::pos2(notice_rect.left() + 28.0, notice_rect.center().y),
                        egui::Align2::LEFT_CENTER,
                        notice,
                        egui::FontId::proportional(12.0),
                        Color32::WHITE,
                    );
                    ui.add_space(18.0);
                }

                // ── Static sections (no scroll) ─────────────────────────────
                // Hero Banner (create project + AI feature card)
                HeroBannerView::render(
                    ui,
                    self.selected_aspect_ratio,
                    &mut trigger_create_modal,
                );
                ui.add_space(14.0);

                // Quick Tools Row
                QuickToolsView::render(ui);
                ui.add_space(14.0);

                // Creative Tools Hub (scrollable strip)
                ToolStripView::render(ui);
                ui.add_space(14.0);

                // ── Project Library (only this scrolls) ─────────────────────
                let library_height = ui.available_height().max(0.0);
                egui::ScrollArea::vertical()
                    .auto_shrink([false, false])
                    .max_height(library_height)
                    .show(ui, |ui| {
                        ProjectListView::render(
                            ui,
                            &mut self.projects,
                            &mut self.view_mode,
                            &mut self.filter_tab,
                            &self.search_query,
                            &mut selected_project_id,
                        );
                    });
            },
        );

        // Handle Project Click
        if let Some(proj_id) = selected_project_id {
            if let Some(p) = self.projects.iter().find(|x| x.id == proj_id) {
                self.opened_project_notice = Some(format!(
                    "Loading project: '{}' into studio timeline…",
                    p.title
                ));
            }
        }

        // Handle Open from Disk dialog via rfd
        if trigger_open_file_dialog {
            if let Some(path) = rfd::FileDialog::new()
                .add_filter("Studio Project", &["vedit", "json"])
                .add_filter("All Files", &["*"])
                .pick_file()
            {
                let filename = path
                    .file_stem()
                    .unwrap_or_default()
                    .to_string_lossy()
                    .to_string();
                self.opened_project_notice =
                    Some(format!("Loaded project file: {}", filename));
            }
        }

        // Handle "+ Create Project" button trigger
        if trigger_create_modal {
            self.new_project_config.aspect_ratio = self.selected_aspect_ratio;
            let (w, h) = self.selected_aspect_ratio.default_resolution();
            self.new_project_config.resolution_w = w;
            self.new_project_config.resolution_h = h;
            self.is_new_project_modal_open = true;
        }

        // Render New Project Modal Dialog
        NewProjectModal::render(
            ui.ctx(),
            &mut self.is_new_project_modal_open,
            &mut self.new_project_config,
            &mut newly_created_project,
        );

        // Append Newly Created Project to list
        if let Some(proj) = newly_created_project {
            self.opened_project_notice = Some(format!(
                "Created new project: '{}' ({})",
                proj.title, proj.resolution_str
            ));
            self.projects.insert(0, proj);
        }
    }
}
