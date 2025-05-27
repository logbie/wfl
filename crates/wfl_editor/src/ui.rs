#[cfg(feature = "gui")]
use egui::{Color32, RichText, Ui};

#[cfg(feature = "gui")]
pub fn title_text(text: &str) -> RichText {
    RichText::new(text)
        .size(18.0)
        .color(Color32::from_rgb(230, 230, 250))
        .strong()
}

#[cfg(feature = "gui")]
pub fn header_text(text: &str) -> RichText {
    RichText::new(text)
        .size(16.0)
        .color(Color32::from_rgb(200, 200, 250))
        .strong()
}

#[cfg(feature = "gui")]
pub fn label_text(text: &str) -> RichText {
    RichText::new(text)
        .size(14.0)
        .color(Color32::from_rgb(180, 180, 250))
}

#[cfg(feature = "gui")]
pub fn error_text(text: &str) -> RichText {
    RichText::new(text)
        .size(14.0)
        .color(Color32::from_rgb(255, 100, 100))
}

#[cfg(feature = "gui")]
pub fn success_text(text: &str) -> RichText {
    RichText::new(text)
        .size(14.0)
        .color(Color32::from_rgb(100, 255, 100))
}

#[cfg(feature = "gui")]
pub fn add_separator(ui: &mut Ui) {
    ui.add_space(5.0);
    ui.separator();
    ui.add_space(5.0);
}
