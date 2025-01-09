use bevy::prelude::*;
use bevy::app::{App, Startup, Update, PluginGroup};
use bevy::asset::AssetServer;
use bevy::DefaultPlugins;
use bevy::input::ButtonState;
use bevy::input::keyboard::KeyboardInput;
use bevy::log::{Level, LogPlugin};
use bevy::window::WindowMode;

use crate::game_state::GameState;
use crate::settings::GameSettings;
use crate::ship::Ship;
use crate::structs::{Cords, COLUMNS, ROWS};

pub struct Game;

const TEXT_COLOR: Color = Color::WHITE;

#[derive(Component)]
struct ScoreText;

#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
enum GameStateWindows {
    #[default]
    Playing,
    Menu,
    GameOver,
}

impl Game {
    pub fn start(&self) {
        let mut game_state = GameState::new();
        Ship::new_fly();
        game_state.game_board.insert(Cords(1, 6), Ship::new_fly());
        game_state.game_board.insert(Cords(0, 7), Ship::new_fly());
        game_state.game_board.insert(Cords(1, 8), Ship::new_fly());
        game_state.game_board.insert(Cords(0, 9), Ship::new_fly());
        game_state.game_board.insert(Cords(1, 10), Ship::new_fly());
        game_state.game_board.insert(Cords(0, 11), Ship::new_fly());
        game_state.game_board.insert(Cords(1, 12), Ship::new_fly());
        game_state.game_board.insert(Cords(0, 13), Ship::new_fly());
        game_state.game_board.insert(Cords(1, 14), Ship::new_fly());

        let mut app = App::new();
        app.add_plugins(
            DefaultPlugins
                .set(LogPlugin {
                    level: Level::DEBUG,
                    filter: "wgpu=error".to_string(),
                    ..default()
                })
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        resizable: false,
                        mode: WindowMode::Windowed,
                        ..default()
                    }),
                    ..default()
                }),
        )
            .insert_resource(game_state)
            .insert_resource(Grid::new(50.0))
            .insert_resource(GameSettings::new())
            .init_state::<GameStateWindows>()

            .add_systems(Startup, crate::background::background_setup)
            .add_systems(Update, crate::background::move_and_respawn_stars)
            .add_systems(Startup, Self::startup)
            .add_systems(
                Update,
                (Self::player_event_listener)
                    .distributive_run_if(in_state(GameStateWindows::Playing)),
            )
            .add_systems(
                Update,
                (Self::update).distributive_run_if(in_state(GameStateWindows::Playing)),
            )
            .add_systems(OnEnter(GameStateWindows::Menu), Self::menu_enter)
            .add_systems(OnEnter(GameStateWindows::GameOver), Self::game_over_enter)
            .add_systems(OnEnter(GameStateWindows::Playing), Self::setup_game_ui)
            .run();
    }

    pub fn startup(mut commands: Commands, asset_server: Res<AssetServer>) {
        commands.spawn(Camera2dBundle::default());
        commands.insert_resource(ClearColor(Color::BLACK));
        commands.spawn(AudioPlayer::new(asset_server.load("sounds/galaga.ogg")));
    }

    pub fn update(
        mut commands: Commands,
        mut game_state: ResMut<GameState>,
        mut grid: ResMut<Grid>,
        asset_server: Res<AssetServer>,
        game_settings: Res<GameSettings>,
    ) {
        grid.despawn_entities(&mut commands);

        game_state.ship_actions().unwrap();
        game_state.player_actions(&game_settings);

        for (&cords, ship) in game_state.game_board.iter() {
            let image_path = ship.display_info();
            let position = (cords.0 as usize, cords.1 as usize);
            grid.add_image_entity(image_path, position);
        }

        if let Some(player_position) = game_state.player.current_position {
            let player_file_path = game_state.player.file_path.clone();
            let position = (player_position.0 as usize, player_position.1 as usize);
            grid.add_image_entity(player_file_path, position);
        }

        for i in 0..game_state.player.lives {
            let position = (ROWS - 1, i as usize);
            grid.add_image_entity(String::from("assets/spaceship.png"), position);
        }

        grid.render_entities(&mut commands, asset_server);
    }

    pub fn player_event_listener(
        mut keyboard_input_events: EventReader<KeyboardInput>,
        mut game_state: ResMut<GameState>,
        mut game_settings: ResMut<GameSettings>,
        mut commands: Commands,
        asset_server: Res<AssetServer>,
    ) {
        if let Some(Cords(x, y)) = game_state.player.current_position {
            for event in keyboard_input_events.read() {
                if event.state == ButtonState::Pressed {
                    match event.key_code {
                        KeyCode::ArrowLeft => {
                            if y > 0 && !game_settings.auto_move {
                                game_state.player.move_to(Cords(x, y - 1));
                            }
                        }
                        KeyCode::ArrowRight => {
                            if y < COLUMNS - 1 && !game_settings.auto_move {
                                game_state.player.move_to(Cords(x, y + 1));
                            }
                        }
                        KeyCode::Space => {
                            if !game_settings.auto_shoot {
                                let bullet_position = Cords(x - 1, y);
                                commands.spawn(
                                    AudioPlayer::new(asset_server.load("sounds/shooting.ogg")),
                                );
                                game_state
                                    .add_ship(bullet_position, Ship::new_bullet(false))
                                    .ok();
                            }
                        }
                        _ => {}
                    }
                }
            }
        }

        game_settings.handle_auto_move(&mut game_state, &Default::default());
        game_settings.handle_auto_shoot(&mut game_state, &mut commands, &Default::default());
    }

    //PAGES
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

    pub fn game_over_enter(mut commands: Commands, asset_server: Res<AssetServer>) {
        commands.spawn(Node {
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            ..default()
        });
    }


    //SCORE TEXT
    pub fn setup_game_ui(mut commands: Commands) {
        commands
            .spawn((
                Node {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    align_items: AlignItems::FlexStart,
                    justify_content: JustifyContent::FlexStart,
                    ..default()
                },
            ))
            .with_children(|parent| {
                parent
                    .spawn((
                        Node {
                            width: Val::Auto,
                            height: Val::Auto,
                            margin: UiRect {
                                left: Val::Px(20.0),
                                top: Val::Px(20.0),
                                right: Val::Px(0.0),
                                bottom: Val::Px(0.0),
                            },
                            ..default()
                        },
                    ))
                    .with_children(|p| {
                        p.spawn((
                            Text::new("Score: 0"),
                            TextFont {
                                font_size: 30.0,
                                ..default()
                            },
                            TextColor(TEXT_COLOR),
                        ));
                    });
            });
    }
}

#[derive(Resource)]
struct Grid {
    cell_size: f32,
    entities: Vec<(String, (usize, usize))>,
    entity_ids: Vec<Entity>,
}

impl Grid {
    fn new(cell_size: f32) -> Self {
        Grid {
            cell_size,
            entities: Vec::new(),
            entity_ids: Vec::new(),
        }
    }

    fn add_image_entity(&mut self, image_path: String, position: (usize, usize)) {
        assert!(
            position.0 < ROWS && position.1 < COLUMNS,
            "Position out of bounds!"
        );
        self.entities.push((image_path, position));
    }

    fn despawn_entities(&mut self, commands: &mut Commands) {
        for entity_id in self.entity_ids.drain(..) {
            commands.entity(entity_id).despawn();
        }
        self.entities.clear();
    }

    fn render_entities(&mut self, commands: &mut Commands, asset_server: Res<AssetServer>) {
        let total_width = COLUMNS as f32 * self.cell_size;
        let total_height = ROWS as f32 * self.cell_size;

        let move_right_offset = (1200.0 - total_width) / 2.0;
        let move_down_offset = (800.0 - total_height) / 2.0;

        for (image_path, (row, col)) in &self.entities {
            let x = *col as f32 * self.cell_size + move_right_offset;
            let y = *row as f32 * self.cell_size + move_down_offset;

            let entity = commands
                .spawn((
                    Node {
                        left: Val::Px(x),
                        top: Val::Px(y),
                        height: Val::Px(self.cell_size),
                        width: Val::Px(self.cell_size),
                        ..default()
                    },
                    ImageNode::new(asset_server.load(image_path)),
                ))
                .id();

            self.entity_ids.push(entity);
        }
    }
}