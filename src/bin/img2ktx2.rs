use anyhow::Result;
use bevy::prelude::*;
use bevy::ecs::hierarchy::ChildSpawnerCommands;
use bevy_basisu_loader::BasisuLoaderPlugin;
fn main() -> Result<()> {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_plugins(BasisuLoaderPlugin)
        .add_systems(Startup, (spawn_camera, spawn_texture_display))
        .run();
    Ok(())
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn((Camera2d::default(), Camera::default(), Transform::default()));
}

fn spawn_texture_display(mut commands: Commands, asset_server: Res<AssetServer>) {
    let png_texture = asset_server.load("imgs/test.png");
    let ktx2_texture = asset_server.load("imgs/test.basisu.ktx2");
    let dds_texture = asset_server.load("imgs/test.dds");
    let default_font = asset_server.load("fonts/simkai.ttf");

    commands
        .spawn(Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            display: Display::Grid,
            grid_template_columns: RepeatedGridTrack::flex(2, 1.0),
            grid_template_rows: RepeatedGridTrack::flex(2, 1.0),
            justify_items: JustifyItems::Center,
            align_items: AlignItems::Center,
            row_gap: Val::Px(20.0),
            column_gap: Val::Px(20.0),
            padding: UiRect::all(Val::Px(20.0)),
            ..default()
        })
        .with_children(|parent| {
            create_texture_card(
                parent,
                "PNG 原图",
                png_texture,
                default_font.clone(),
            );
            create_texture_card(
                parent,
                "KTX2 (BasisU)",
                ktx2_texture,
                default_font.clone(),
            );
            create_texture_card(
                parent,
                "DDS 纹理",
                dds_texture,
                default_font.clone(),
            );
        });
}

fn create_texture_card(
    parent: &mut ChildSpawnerCommands,
    title: &str,
    texture: Handle<Image>,
    font: Handle<Font>,
) {
    parent
        .spawn(Node {
            flex_direction: FlexDirection::Column,
            align_items: AlignItems::Center,
            row_gap: Val::Percent(10.0),
            justify_content: JustifyContent::Center,
            width: Val::Percent(90.0),
            height: Val::Percent(90.0),
            ..default()
        })
        .with_children(|p| {
            p.spawn((
                Text::new(title),
                TextFont {
                    font: FontSource::Handle(font.clone()),
                    font_size: FontSize::Px(24.0),
                    ..default()
                },
                TextColor(Color::WHITE),
            ));

            p.spawn((
                ImageNode::new(texture),
                Node {
                    width: Val::Percent(80.0),
                    max_width: Val::Px(400.0),
                    aspect_ratio: Some(1.0),
                    ..default()
                },
            ));
        });
}