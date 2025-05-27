use egui::{Context, Visuals};

pub struct Theme {
    pub is_dark: bool,
    
    visuals: Visuals,
}

impl Theme {
    pub fn dark() -> Self {
        Self {
            is_dark: true,
            visuals: Visuals::dark(),
        }
    }
    
    pub fn light() -> Self {
        Self {
            is_dark: false,
            visuals: Visuals::light(),
        }
    }
    
    pub fn apply(&self, ctx: Context) {
        ctx.set_visuals(self.visuals.clone());
    }
}
