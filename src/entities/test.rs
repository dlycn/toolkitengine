use bevy::prelude::*;
pub fn mesh2d(size:f32,color:Srgba)->impl Scene{
    bsn!{
        Mesh2d(asset_value(Rectangle::new(size, size)))
        MeshMaterial2d::<ColorMaterial>(asset_value(Color::from(color)))
    }
}