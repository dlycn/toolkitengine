use bevy::prelude::*;
pub fn new(asset_server: &AssetServer,path:String) -> impl Bundle {
    (ImageNode::new(asset_server.load(path)).with_mode(NodeImageMode::Stretch),
    Node{
        display:Display::Flex,
        left: Val::Percent(0.),
        top: Val::Percent(0.),
        height: Val::Px(64.),
        overflow: Overflow::scroll(),
        aspect_ratio: Some(1.0),
        ..default()
    })
}