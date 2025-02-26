use bevy::asset::AssetServer;
use bevy::prelude::{default, AlignItems, Commands, ImageNode, JustifyContent, Node, Res, Val};

pub fn menu_enter(mut commands: Commands, asset_server: Res<AssetServer>) {
    let icon = asset_server.load("assets/logo.png");

    commands
        .spawn(Node {
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            ..default()
        })
        .with_children(|parent| {
            parent.spawn((
                ImageNode::new(icon),
                Node {
                    width: Val::Px(200.0),
                    ..default()
                },
            ));
        });
}