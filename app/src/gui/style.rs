use eframe::{
    egui::{style::Selection, Visuals},
    epaint::{Color32, Stroke, Vec2},
};

pub struct Theme {
    pub spacing: SpacingStyle,
    pub visuals: Visuals,

    // Liquid glass primitives
    pub glass_color: Color32,
    pub accent_color: Color32,
    pub glass_corner_radius: f32,
    pub glass_blur: f32,
}

impl Default for Theme {
    fn default() -> Self {
        Self {
            spacing: SpacingStyle::default(),
            visuals: Visuals {
                selection: Selection {
                    bg_fill: Color32::from_rgb(40, 100, 160),
                    stroke: Stroke::new(1.0, Color32::from_rgb(192, 222, 255)),
                },
                ..Visuals::default()
            },

            // Glass look defaults (subtle, translucent)
            glass_color: Color32::from_rgba_unmultiplied(28, 32, 40, 180),
            accent_color: Color32::from_rgb(90, 160, 220),
            glass_corner_radius: 8.0,
            glass_blur: 10.0,
        }
    }
}

impl Theme {
    // Fill color for glass panels
    pub fn glass_fill(&self) -> Color32 {
        self.glass_color
    }

    // Stroke used for glass edges / outlines
    pub fn glass_stroke(&self) -> Stroke {
        Stroke::new(1.0, self.accent_color)
    }

    // Corner radius for glass frames
    pub fn glass_radius(&self) -> f32 {
        self.glass_corner_radius
    }

    // Suggested blur radius (for later integration with custom painters)
    pub fn glass_blur_radius(&self) -> f32 {
        self.glass_blur
    }
}

pub struct SpacingStyle {
    pub default: Vec2,
    pub large: f32,
    pub medium: f32,
}

impl Default for SpacingStyle {
    fn default() -> Self {
        Self {
            default: Vec2::new(8.0, 5.0),
            large: 20.0,
            medium: 8.0,
        }
    }
}
