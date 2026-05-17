use egui::Margin;

#[derive(Default)]
pub enum Size {
    Small,
    #[default]
    Medium,
}

impl Size {
    pub fn icon_size(&self) -> f32 {
        match self {
            Size::Small => 18.0,
            Size::Medium => 30.0,
        }
    }

    pub fn inner_margin(&self) -> Margin {
        match self {
            Size::Small => egui::Margin::symmetric(5, 5),
            Size::Medium => egui::Margin::symmetric(10, 10),
        }
    }
}
