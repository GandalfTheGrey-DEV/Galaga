use bevy::prelude::*;
use crate::settings::GameSettings;

#[derive(Component)]
pub struct SettingsDisplay;

pub fn spawn_or_update_settings_display(
    mut commands: Commands,
    game_settings: Res<GameSettings>,
    mut query: Query<Entity, With<SettingsDisplay>>,
) {
    // First, check if there is an existing `SettingsDisplay` and despawn it
    if let Ok(entity) = query.get_single_mut() {
        commands.entity(entity).despawn_recursive();
    }

    // Spawn a new settings display
    commands
        .spawn((
            SettingsDisplay,
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                flex_direction: FlexDirection::Row,
                align_items: AlignItems::FlexStart,
                justify_content: JustifyContent::FlexEnd,
                ..default()
            },
        ))
        .with_children(|parent| {
            spawn_settings_column(
                parent,
                "Keyboard",
                &[
                    ("Invincible (D)", game_settings.invocable),
                    ("Auto Pilot (M)", game_settings.auto_pilot),
                    ("Auto Shoot (S)", game_settings.auto_shoot),
                    ("Fly Move (A)", game_settings.fly_move),
                    ("Laser Shoot (W)", game_settings.laser_shoot),
                ],
            );

            spawn_settings_column(
                parent,
                "Xbox",
                &[
                    ("Invincible (Y)", game_settings.invocable),
                    ("Auto Pilot (X)", game_settings.auto_pilot),
                    ("Auto Shoot (A)", game_settings.auto_shoot),
                    ("Fly Move (B)", game_settings.fly_move),
                    ("Laser Shoot (D-pad right)", game_settings.laser_shoot),
                ],
            );
        });
}
//TODO make grid smaller
//TODO fix name on invocalbe
fn spawn_settings_column(
    parent: &mut ChildBuilder,
    title: &str,
    settings: &[(&str, bool)],
) {
    parent
        .spawn(Node {
            width: Val::Auto,
            height: Val::Percent(100.0),
            flex_direction: FlexDirection::Column,
            align_items: AlignItems::FlexStart,
            margin: UiRect {
                right: Val::Px(20.0),
                top: Val::Px(20.0),
                ..default()
            },
            ..default()
        })
        .with_children(|column| {
            column.spawn((
                Text::new(title),
                TextFont {
                    font_size: 20.0,
                    ..default()
                },
                TextColor(Color::rgba(1.0, 1.0, 1.0, 1.0)),
            ));

            for (label, is_on) in settings {
                let text_color = if *is_on {
                    TextColor(Color::rgba(0.0, 1.0, 0.0, 1.0))
                } else {
                    TextColor(Color::rgba(1.0, 1.0, 0.0, 1.0))
                };

                column.spawn((
                    Text::new(format!(
                        "{}: {}",
                        if *is_on { "(ON)" } else { "(OFF)" },
                        label
                    )),
                    TextFont {
                        font_size: 15.0,
                        ..default()
                    },
                    text_color,
                ));
            }
        });
}