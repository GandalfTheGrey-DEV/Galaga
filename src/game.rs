use bevy::app::{App, PluginGroup, Startup, Update};
use bevy::asset::AssetServer;
use bevy::audio::AudioPlayer;
use bevy::color::Color;
use bevy::DefaultPlugins;
use bevy::input::ButtonInput;
use bevy::input::ButtonState;
use bevy::input::keyboard::KeyboardInput;
use bevy::log::{Level, LogPlugin};
use bevy::prelude::{default, Camera2dBundle, ClearColor, Commands, Entity, EventReader, ImageNode, KeyCode, Node, Query, Res, ResMut, Resource, Val, Window, WindowPlugin};
use bevy::window::WindowMode;
use crossterm::event::KeyCode::Menu;
use crate::game_state::GameState;
use crate::ship::Ship;
use crate::structs::{Cords, COLUMNS, ROWS};

pub struct Game;

impl Game {
    pub fn start(&self) {
        let mut games_state = GameState::new();

        games_state.game_board.insert(Cords(0, 0), Ship::new_fly());
        games_state.game_board.insert(Cords(1, 1), Ship::new_fly());
        games_state.game_board.insert(Cords(0, 2), Ship::new_fly());
        games_state.game_board.insert(Cords(1, 3), Ship::new_fly());
        games_state.game_board.insert(Cords(0, 4), Ship::new_fly());
        games_state.game_board.insert(Cords(1, 5), Ship::new_fly());
        games_state.game_board.insert(Cords(0, 6), Ship::new_fly());
        games_state.game_board.insert(Cords(1, 7), Ship::new_fly());
        games_state.game_board.insert(Cords(0, 8), Ship::new_fly());
        games_state.game_board.insert(Cords(1, 9), Ship::new_fly());

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

            .insert_resource(games_state)
            .insert_resource(Grid::new(50.0))
            .add_systems(Startup, Self::startup)
            .add_systems(Update, Self::player_event_listener)
            .add_systems(Update, Self::update)
            .run();
    }
   pub fn startup(asset_server: Res<AssetServer>, mut commands: Commands) {
        commands.spawn(AudioPlayer::new(asset_server.load("sounds/galaga.ogg")));
        commands.spawn(Camera2dBundle::default());
        commands.insert_resource(ClearColor(Color::BLACK));
    }

    pub fn player_event_listener(
        mut keyboard_input_events: EventReader<KeyboardInput>,
        mut game_state: ResMut<GameState>,
        keys: Res<ButtonInput<KeyCode>>,
        asset_server: Res<AssetServer>,
        mut commands: Commands,

    ) {
        if let Some(Cords(x, y)) = game_state.player.current_position {
            for event in keyboard_input_events.read() {
                if event.state == ButtonState::Pressed {
                     match event.key_code {
                        KeyCode::ArrowLeft  => {
                            if y > 0 {
                                game_state.player.move_to(Cords(x, y - 1));
                            }
                        }
                        KeyCode::ArrowRight => {
                            if y < COLUMNS - 1 {
                                game_state.player.move_to(Cords(x, y + 1));
                            }
                        }
                        KeyCode::Space => {
                            let bullet_position = Cords(x - 1, y);
                            commands.spawn(AudioPlayer::new(asset_server.load("sounds/shoot.ogg")));
                            game_state.add_ship(bullet_position, Ship::new_bullet(false)).ok();
                        }
                        _ => {}

                    }
                }
            }
        }
    }

    pub fn update(

        mut commands: Commands,
        mut game_state: ResMut<GameState>,
        mut grid: ResMut<Grid>,
        asset_server: Res<AssetServer>,
    ) {
        grid.despawn_entities(&mut commands);

        game_state.ship_actions().unwrap();
        game_state.player_actions();
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

        grid.render_entities(&mut commands, asset_server);
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

            let entity = commands.spawn((
                Node {
                    left: Val::Px(x),
                    top: Val::Px(y),
                    height: Val::Px(self.cell_size),
                    width: Val::Px(self.cell_size),
                    ..default()
                },
                ImageNode::new(asset_server.load(image_path)),
            )).id();

            self.entity_ids.push(entity);
        }
    }
}
