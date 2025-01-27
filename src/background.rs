use bevy::{prelude::*, sprite::SpriteBundle, window::PrimaryWindow};
use rand::Rng;

#[derive(Component)]
pub struct Star;

pub fn background_setup(mut commands: Commands, windows: Query<&Window, With<PrimaryWindow>>) {

    commands.insert_resource(ClearColor(Color::rgb(0.0, 0.0, 0.0)));

    let window = windows.single();

    let star_count = 100;
    let screen_width = window.width();
    let screen_height = window.height();

    let mut rng = rand::thread_rng();

    for _ in 0..star_count {
        let position = Vec3::new(
            rng.gen_range(-screen_width / 2.0..screen_width / 2.0),
            rng.gen_range(-screen_height / 2.0..screen_height / 2.0),
            0.0,
        );

        let star_colors = vec![
            Color::rgb(1.0, 1.0, 1.0),
            Color::rgb(1.0, 1.0, 0.5),
            Color::rgb(0.5, 0.5, 1.0),
        ];

        let random_color = star_colors[rng.gen_range(0..star_colors.len())];

        let size = rng.gen_range(2.0..6.0);

        commands.spawn((
            SpriteBundle {
                sprite: Sprite {
                    color: random_color,
                    custom_size: Some(Vec2::new(size, size)),
                    ..default()
                },
                transform: Transform::from_translation(position),
                ..default()
            },
            Star,
        ));
    }
}

pub fn move_and_respawn_stars(
    mut query: Query<&mut Transform, With<Star>>,
    windows: Query<&Window, With<PrimaryWindow>>,
) {
    let window = windows.single();
    let screen_width = window.width();
    let screen_height = window.height();

    let mut rng = rand::thread_rng();

    for mut transform in query.iter_mut() {
        transform.translation.y -= rng.gen_range(0.1..1.0);

        if transform.translation.y < -screen_height / 2.0 {
            transform.translation.y = screen_height / 2.0;
            transform.translation.x = rng.gen_range(-screen_width / 2.0..screen_width / 2.0);
        }
    }
}
