use crate::models::{AspectRatio, FrameRatePreset, ProjectItem};
use crate::theme::StudioColors;
use eframe::egui::{self, Color32, CornerRadius, Vec2, Window};

#[derive(Debug, Clone)]
pub struct NewProjectConfig {
    pub title: String,
    pub aspect_ratio: AspectRatio,
    pub resolution_w: u32,
    pub resolution_h: u32,
    pub frame_rate: FrameRatePreset,
    pub working_color_space: String,
    pub destination_path: String,
}

impl Default for NewProjectConfig {
    fn default() -> Self {
        Self {
            title: "Untitled Studio Project".to_string(),
            aspect_ratio: AspectRatio::Widescreen16x9,
            resolution_w: 3840,
            resolution_h: 2160,
            frame_rate: FrameRatePreset::Fps24,
            working_color_space: "ACEScct".to_string(),
            destination_path: "~/Movies/StudioProjects".to_string(),
        }
    }
}

pub struct NewProjectModal;

impl NewProjectModal {
    pub fn render(
        ctx: &egui::Context,
        is_open: &mut bool,
        config: &mut NewProjectConfig,
        on_create: &mut Option<ProjectItem>,
    ) {
        if !*is_open {
            return;
        }

        let mut close_modal = false;

        Window::new("🎬 Create New Studio Project")
            .collapsible(false)
            .resizable(false)
            .fixed_size(Vec2::new(480.0, 420.0))
            .anchor(egui::Align2::CENTER_CENTER, Vec2::ZERO)
            .show(ctx, |ui| {
                ui.add_space(8.0);

                // 1. Project Title
                ui.label(egui::RichText::new("Project Name").strong().size(12.0));
                ui.add(
                    egui::TextEdit::singleline(&mut config.title)
                        .desired_width(ui.available_width())
                        .hint_text("Enter project title..."),
                );

                ui.add_space(12.0);

                // 2. Aspect Ratio Presets
                ui.label(
                    egui::RichText::new("Aspect Ratio & Delivery Target")
                        .strong()
                        .size(12.0),
                );
                ui.horizontal(|ui| {
                    let ratios = [
                        (AspectRatio::Widescreen16x9, "16:9 Landscape"),
                        (AspectRatio::Vertical9x16, "9:16 Vertical"),
                        (AspectRatio::Square1x1, "1:1 Square"),
                        (AspectRatio::Ultrawide21x9, "21:9 Cinema"),
                    ];

                    for (ratio, label) in ratios {
                        let is_selected = config.aspect_ratio == ratio;
                        if ui.selectable_label(is_selected, label).clicked() {
                            config.aspect_ratio = ratio;
                            let (w, h) = ratio.default_resolution();
                            config.resolution_w = w;
                            config.resolution_h = h;
                        }
                    }
                });

                ui.add_space(12.0);

                // 3. Canvas Resolution
                ui.label(
                    egui::RichText::new("Timeline Resolution")
                        .strong()
                        .size(12.0),
                );
                ui.horizontal(|ui| {
                    ui.label("Width:");
                    ui.add(
                        egui::DragValue::new(&mut config.resolution_w)
                            .range(480..=7680)
                            .speed(10.0),
                    );
                    ui.label("px  ×  Height:");
                    ui.add(
                        egui::DragValue::new(&mut config.resolution_h)
                            .range(480..=4320)
                            .speed(10.0),
                    );
                    ui.label("px");
                });

                ui.add_space(12.0);

                // 4. Timebase & Working Color Space
                ui.columns(2, |cols| {
                    cols[0].vertical(|ui| {
                        ui.label(egui::RichText::new("Timebase / FPS").strong().size(12.0));
                        egui::ComboBox::from_id_salt("fps_combo")
                            .selected_text(config.frame_rate.label())
                            .show_ui(ui, |ui| {
                                let fps_options = [
                                    FrameRatePreset::Fps24,
                                    FrameRatePreset::Fps25,
                                    FrameRatePreset::Fps30,
                                    FrameRatePreset::Fps60,
                                    FrameRatePreset::Fps120,
                                ];
                                for fps in fps_options {
                                    ui.selectable_value(&mut config.frame_rate, fps, fps.label());
                                }
                            });
                    });

                    cols[1].vertical(|ui| {
                        ui.label(egui::RichText::new("Color Management").strong().size(12.0));
                        egui::ComboBox::from_id_salt("cs_combo")
                            .selected_text(&config.working_color_space)
                            .show_ui(ui, |ui| {
                                ui.selectable_value(
                                    &mut config.working_color_space,
                                    "ACEScct".to_string(),
                                    "ACEScct (Cinema)",
                                );
                                ui.selectable_value(
                                    &mut config.working_color_space,
                                    "ACEScg".to_string(),
                                    "ACEScg (VFX)",
                                );
                                ui.selectable_value(
                                    &mut config.working_color_space,
                                    "Rec.709".to_string(),
                                    "Rec.709 (Standard Web)",
                                );
                                ui.selectable_value(
                                    &mut config.working_color_space,
                                    "Apple Log".to_string(),
                                    "Apple Log / Rec.2020",
                                );
                            });
                    });
                });

                ui.add_space(16.0);

                // 5. Storage Location
                ui.label(egui::RichText::new("Project Folder").strong().size(12.0));
                ui.horizontal(|ui| {
                    ui.add(
                        egui::TextEdit::singleline(&mut config.destination_path)
                            .desired_width(ui.available_width() - 85.0),
                    );
                    if ui.button("Browse...").clicked() {
                        if let Some(folder) = rfd::FileDialog::new().pick_folder() {
                            config.destination_path = folder.to_string_lossy().to_string();
                        }
                    }
                });

                ui.add_space(24.0);
                ui.separator();
                ui.add_space(10.0);

                // 6. Action Buttons
                ui.horizontal(|ui| {
                    if ui.button("Cancel").clicked() {
                        close_modal = true;
                    }

                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        let create_btn = egui::Button::new(
                            egui::RichText::new("🚀  Create Project")
                                .strong()
                                .color(Color32::WHITE),
                        )
                        .fill(StudioColors::ACCENT_BLUE)
                        .min_size(Vec2::new(140.0, 32.0))
                        .corner_radius(CornerRadius::same(6));

                        if ui.add(create_btn).clicked() {
                            let new_item = ProjectItem {
                                id: format!(
                                    "proj-{}",
                                    uuid::Uuid::new_v4().to_string()[..8].to_string()
                                ),
                                title: config.title.clone(),
                                aspect_ratio: config.aspect_ratio,
                                resolution_str: format!(
                                    "{}x{}",
                                    config.resolution_w, config.resolution_h
                                ),
                                duration_str: "00:00".to_string(),
                                size_str: "12 KB".to_string(),
                                last_modified: "Just now".to_string(),
                                is_cloud_synced: false,
                                thumbnail_accent: (30, 140, 180),
                            };
                            *on_create = Some(new_item);
                            close_modal = true;
                        }
                    });
                });
            });

        if close_modal {
            *is_open = false;
        }
    }
}
