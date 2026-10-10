use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AspectRatio {
    Widescreen16x9,
    Vertical9x16,
    Square1x1,
    Portrait4x5,
    Ultrawide21x9,
}

impl AspectRatio {
    pub fn default_resolution(&self) -> (u32, u32) {
        match self {
            AspectRatio::Widescreen16x9 => (3840, 2160),
            AspectRatio::Vertical9x16 => (1080, 1920),
            AspectRatio::Square1x1 => (1080, 1080),
            AspectRatio::Portrait4x5 => (1080, 1350),
            AspectRatio::Ultrawide21x9 => (3440, 1440),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum FrameRatePreset {
    Fps24,
    Fps25,
    Fps30,
    Fps60,
    Fps120,
}

impl FrameRatePreset {
    pub fn label(&self) -> &'static str {
        match self {
            FrameRatePreset::Fps24 => "24 fps (Cinema)",
            FrameRatePreset::Fps25 => "25 fps (PAL)",
            FrameRatePreset::Fps30 => "30 fps (Web/TV)",
            FrameRatePreset::Fps60 => "60 fps (Smooth)",
            FrameRatePreset::Fps120 => "120 fps (HFR)",
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectItem {
    pub id: String,
    pub title: String,
    pub aspect_ratio: AspectRatio,
    pub resolution_str: String,
    pub duration_str: String,
    pub size_str: String,
    pub last_modified: String,
    pub is_cloud_synced: bool,
    pub thumbnail_accent: (u8, u8, u8), // RGB seed for procedural mock thumbnail
}

impl ProjectItem {
    pub fn mock_projects() -> Vec<Self> {
        vec![
            Self {
                id: "proj-001".to_string(),
                title: "Cinematic Travel Vlog - Kyoto Autumn".to_string(),
                aspect_ratio: AspectRatio::Widescreen16x9,
                resolution_str: "4K UHD".to_string(),
                duration_str: "08:42".to_string(),
                size_str: "1.42 GB".to_string(),
                last_modified: "Today, 14:18".to_string(),
                is_cloud_synced: true,
                thumbnail_accent: (45, 110, 140),
            },
            Self {
                id: "proj-002".to_string(),
                title: "Streetwear Fashion Drop Reel".to_string(),
                aspect_ratio: AspectRatio::Vertical9x16,
                resolution_str: "1080x1920".to_string(),
                duration_str: "00:45".to_string(),
                size_str: "215 MB".to_string(),
                last_modified: "Yesterday, 18:30".to_string(),
                is_cloud_synced: true,
                thumbnail_accent: (140, 50, 110),
            },
            Self {
                id: "proj-003".to_string(),
                title: "Commercial Product Promo - Wireless ANC".to_string(),
                aspect_ratio: AspectRatio::Widescreen16x9,
                resolution_str: "4K DCI".to_string(),
                duration_str: "01:30".to_string(),
                size_str: "890 MB".to_string(),
                last_modified: "Oct 8, 2026".to_string(),
                is_cloud_synced: false,
                thumbnail_accent: (160, 110, 40),
            },
            Self {
                id: "proj-004".to_string(),
                title: "Podcast Interview Snippets #14".to_string(),
                aspect_ratio: AspectRatio::Vertical9x16,
                resolution_str: "1080x1920".to_string(),
                duration_str: "02:15".to_string(),
                size_str: "420 MB".to_string(),
                last_modified: "Oct 6, 2026".to_string(),
                is_cloud_synced: true,
                thumbnail_accent: (40, 140, 95),
            },
            Self {
                id: "proj-005".to_string(),
                title: "Indie Documentary Rough Cut Ep 02".to_string(),
                aspect_ratio: AspectRatio::Ultrawide21x9,
                resolution_str: "3.4K Scope".to_string(),
                duration_str: "24:10".to_string(),
                size_str: "6.85 GB".to_string(),
                last_modified: "Oct 2, 2026".to_string(),
                is_cloud_synced: false,
                thumbnail_accent: (120, 60, 60),
            },
            Self {
                id: "proj-006".to_string(),
                title: "Social Media Square Carousel Teaser".to_string(),
                aspect_ratio: AspectRatio::Square1x1,
                resolution_str: "1080x1080".to_string(),
                duration_str: "00:30".to_string(),
                size_str: "95 MB".to_string(),
                last_modified: "Sep 28, 2026".to_string(),
                is_cloud_synced: true,
                thumbnail_accent: (85, 80, 160),
            },
        ]
    }
}
