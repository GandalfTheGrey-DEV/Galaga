use bevy::asset::AssetServer;
use bevy::prelude::{default, AlignItems, Commands, JustifyContent, Node, Res, Val};
use bevy::{prelude::*};

pub fn game_over_enter(mut commands: Commands, asset_server: Res<AssetServer>, query: Query<Entity>, game_state: Res<crate::game_state::GameState>) {

    // Create the root node
    commands.spawn(Node {
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        width: Val::Percent(100.0),
        height: Val::Percent(100.0),
        ..default()
    });

    commands
        .spawn(Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            flex_direction: FlexDirection::Column,
            ..default()
        })
        .with_children(|parent| {
            parent
                .spawn((
                    Button,
                    Node {
                        width: Val::Px(150.0),
                        height: Val::Px(65.0),
                        margin: UiRect {
                            top: Val::Px(5.0),
                            bottom: Val::Px(5.0),
                            left: Val::Px(0.0),
                            right: Val::Px(0.0),
                        },
                        border: UiRect::all(Val::Px(2.0)),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    BorderColor(Color::BLACK),
                    BorderRadius::new(
                        Val::Px(10.),
                        Val::Px(10.),
                        Val::Px(10.),
                        Val::Px(10.),
                    ),
                ))
                .with_child((
                    Text::new("Restart"),
                    TextFont {
                        font: asset_server.load("fonts/FiraSans-ExtraBold.ttf"),
                        font_size: 20.0,
                        ..default()
                    },
                    TextColor(Color::srgb(0.9, 0.9, 0.9)),
                ));

            parent
                .spawn((
                    Button,
                    Node {
                        width: Val::Px(150.0),
                        height: Val::Px(65.0),
                        margin: UiRect {
                            top: Val::Px(5.0),
                            bottom: Val::Px(5.0),
                            left: Val::Px(0.0),
                            right: Val::Px(0.0),
                        },
                        border: UiRect::all(Val::Px(2.0)),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    BorderColor(Color::BLACK),
                    BorderRadius::new(
                        Val::Px(10.),
                        Val::Px(10.),
                        Val::Px(10.),
                        Val::Px(10.),
                    ),
                ))
                .with_child((
                    Text::new("Menu"),
                    TextFont {
                        font: asset_server.load("fonts/FiraSans-ExtraBold.ttf"),
                        font_size: 20.0,
                        ..default()
                    },
                    TextColor(Color::srgb(0.9, 0.9, 0.9)),
                ));
        });
}

const NORMAL_BUTTON: Color = Color::srgb(0.15, 0.15, 0.15);
const HOVERED_BUTTON: Color = Color::srgb(0.25, 0.25, 0.25);
const PRESSED_BUTTON: Color = Color::srgb(0.35, 0.75, 0.35);

pub(crate) fn gameover_button_system(
    mut interaction_query: Query<
        (
            &Interaction,
            &mut BackgroundColor,
            &mut BorderColor,
            &Children,
        ),
        (Changed<Interaction>, With<Button>),
    >,
    mut text_query: Query<&mut Text>,
) {
    for (interaction, mut color, mut border_color, children) in &mut interaction_query {
        let mut text = text_query.get_mut(children[0]).unwrap();
        match *interaction {
            Interaction::Pressed => {

            }
            Interaction::Hovered => {
                *color = HOVERED_BUTTON.into();
                border_color.0 = Color::WHITE;
            }
            Interaction::None => {
                *color = NORMAL_BUTTON.into();
                border_color.0 = Color::BLACK;
            }
        }
    }
}
