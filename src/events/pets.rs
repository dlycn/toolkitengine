use std::collections::HashMap;

use crate::components::PetConfig;
use crate::configs::res::pet::*;
use crate::entities::e2s;
use bevy::prelude::*;

#[derive(EntityEvent)]
pub struct ESelectPet {
    pub entity: Entity,
}

pub fn e_select_pet(event: On<ESelectPet>, mut respet: ResMut<SelectedPet>) {
    respet.0 = Some(event.entity);
    debug!("{:?}", respet.0);
}

#[derive(Event)]
pub struct ECreatePet {
    pub idnum: Vec<([usize; 2], fn(usize) -> Vec2)>,
}

pub fn e_create_pet(
    event: On<ECreatePet>,

    asset_server: Res<AssetServer>,
    mut commands: Commands,
    mut respet: ResMut<CreatedPet>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    let mut pet_config = HashMap::new();
    pet_config.insert(
        5000,
        PetConfig {
            texture_size: [256, 224],
            pet_name: "圣灵谱尼".to_string(),
            pet_indice: 26,
        },
    );
    pet_config.insert(
        3414,
        PetConfig {
            texture_size: [256, 237],
            pet_name: "混沌魔君索伦森".to_string(),
            pet_indice: 26,
        },
    );

    for (param, func) in event.idnum.iter() {
        let id = param[0];
        let num = param[1];
        let data = pet_config.get(&id).unwrap();
        let size = data.texture_size;
        let name = data.pet_name.clone();
        let indice = data.pet_indice;
        let texture = asset_server.load(format!("imgs/{}.basisu.ktx2", id));
        let array = UVec2::new(size[0], size[1]);
        let layout = TextureAtlasLayout::from_grid(array, 8, 4, None, None);

        let texture_atlas_layout = texture_atlas_layouts.add(layout);

        // 在 for (param, func) in event.idnum.iter() 循环内部：
        let mut pets: Vec<_> = (0..num)
            .map(|i| {
                e2s::pet::stdpet(
                    texture.clone(),
                    texture_atlas_layout.clone(),
                    indice,
                    name.clone(),
                    Vec2::new(size[0] as f32, size[1] as f32),
                    func(i),
                )
            })
            .collect();

        let created_pet = pets.pop();

        commands.queue_spawn_scene_list(pets);

        respet.0 = Some(commands.spawn_scene(created_pet).id());
    }
}
