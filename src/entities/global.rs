use bevy::{prelude::*, text::FontSourceTemplate};
pub fn font(size:f32,color:Color) -> impl Scene {
    bsn! {
    TextFont {
        font: FontSourceTemplate::Handle("fonts/source han sans.otf"),
        font_size: FontSize::Px(size),
        style: FontStyle::Normal,
    }
    TextColor(color)
    }
}

pub fn std_font() -> impl Scene {
    font(24.0,Color::BLACK)
}   