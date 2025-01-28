use bevy::{
    prelude::{default, Commands, Component, Query, Res, Text, TextColor, TextFont, Time, Timer, TimerMode, With},
};
use crate::game_state::GameState;

#[derive(Component)]
pub struct ScoreText;

#[derive(Component)]
pub struct FontSizeAnimation {
    timer: Timer,
    last_score: i32,
}

pub fn setup_score_ui(mut commands: Commands) {
    commands.spawn((
        Text::new("Score: 0"),
        TextFont {
            font_size: 25.0,
            ..default()
        },
        TextColor(bevy::prelude::Color::rgba(0.0, 0.0, 1.0, 1.0)),
        ScoreText,
        FontSizeAnimation {
            timer: Timer::from_seconds(0.2, TimerMode::Once),
            last_score: 0,
        },
    ));
}

pub fn update_score_ui(
    time: Res<Time>,
    game_state: Res<GameState>,
    mut query: Query<(&mut Text, &mut TextFont, &mut FontSizeAnimation), With<ScoreText>>,
) {
    if let Ok((mut text, mut font, mut animation)) = query.get_single_mut() {
        if game_state.score >= (animation.last_score + 100) as u32 {
            animation.last_score = game_state.score as i32;
            animation.timer.reset();
            font.font_size = 30.0;
        }

        *text = Text::new(format!("Score: {}", game_state.score));

        animation.timer.tick(time.delta());
        if animation.timer.finished() {
            font.font_size = 25.0;
        }
    }
}
