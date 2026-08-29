use bevy::{prelude::*, render::render_resource::AsBindGroup, shader::ShaderRef, sprite_render::Material2d};

#[derive(AsBindGroup, Asset, TypePath, Debug, Clone)]
pub struct HpMaterial {
    #[uniform(0)]
    pub color: LinearRgba,
}

impl UiMaterial for HpMaterial {
    fn fragment_shader() -> ShaderRef {
        "libs/hpbar.wgsl".into()
    }
}

#[derive(AsBindGroup, Asset, TypePath, Debug, Clone)]
pub struct ExpMaterial {
    #[uniform(0)]
    pub color: LinearRgba,
}

impl UiMaterial for ExpMaterial {
    fn fragment_shader() -> ShaderRef {
        "libs/expbar.wgsl".into()
    }
}

#[derive(AsBindGroup, Asset, TypePath, Debug, Clone)]
pub struct StateMaterial {
    #[uniform(0)]
    pub color: LinearRgba,
}

impl UiMaterial for StateMaterial {
    fn fragment_shader() -> ShaderRef {
        "libs/stategroup.wgsl".into()
    }
}



#[derive(AsBindGroup, Asset, TypePath, Debug, Clone)]
pub struct AreaMaterial {
    #[uniform(0)]
    pub color: LinearRgba,
}

impl Material2d for AreaMaterial {
    fn fragment_shader() -> ShaderRef {
        "libs/area.wgsl".into()
    }
}

#[derive(AsBindGroup, Asset, TypePath, Debug, Clone)]
pub struct MapMaterial {
    #[uniform(0)]
    pub color: LinearRgba,
}

impl UiMaterial for MapMaterial {
    fn fragment_shader() -> ShaderRef {
        "libs/minimap.wgsl".into()
    }
}