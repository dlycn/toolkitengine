use bevy::prelude::*;
use bevy::input::mouse::AccumulatedMouseScroll;
pub struct HungrySnakePlugin;

use crate::configs::res;

const SIZE: f32 = 1000.0;

fn ui()->impl SceneList {
    bsn_list!(
        (
            Camera2d
            Camera{clear_color: ClearColorConfig::Custom(Color::NONE),}
        ),
        (
            Mesh2d(asset_value(Rectangle::new(SIZE, SIZE)))
            MeshMaterial2d::<ColorMaterial>(asset_value(Color::WHITE))
        )
    )
}

fn control(
    mut commands: Commands,
    mut cameraquery: Query<(&mut Transform, &mut Projection),With<Camera2d>>,
    input: Res<ButtonInput<KeyCode>>,
    scroll: Res<AccumulatedMouseScroll>,
    time: Res<Time>,
){

}

impl Plugin for HungrySnakePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, ui.spawn());
    }
}
