use bevy::{prelude::*, text::FontSourceTemplate};
pub fn font(size:f32,color:Color) -> impl Scene {
    bsn! {
    TextFont {
        font: FontSourceTemplate::Handle("fonts/SourceHanSansCN-Heavy.otf"),
        font_size: FontSize::Rem(size),
        style: FontStyle::Normal,
    }
    TextColor(color)
    }
}

pub fn std_font(scale:f32) -> impl Scene {
    let size = scale;
    font(size,Color::BLACK)
}   