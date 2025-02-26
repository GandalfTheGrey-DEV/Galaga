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
use std::default::Default;
use std::sync::{Arc, Mutex};
use std::thread;
use tokio::sync::mpsc;
use crate::ArduinoAction;
use crate::ship::Ship;
use crate::structs::{Cords, COLUMNS, ROWS};

// New resource wrapper for the Arduino receiver
#[derive(Resource)]
pub struct ArduinoReceiver {
    pub receiver: Arc<Mutex<mpsc::Receiver<ArduinoAction>>>
}

pub struct Game {
    pub(crate) rx: Arc<Mutex<mpsc::Receiver<ArduinoAction>>>,
}

use crate::game_over::{game_over_enter, gameover_button_system};
use crate::score_display::{setup_score_ui, update_score_ui};
use crate::settings_display::spawn_or_update_settings_display;

pub(crate) const TEXT_COLOR: Color = Color::WHITE;

#[derive(Component)]
pub(crate) struct ScoreText;

#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
enum GameStateWindows {
    #[default]
    Playing,
    GameOver,
    Menu,
}

impl Game {
    pub fn process_arduino_actions(
        arduino_receiver: Res<ArduinoReceiver>,
        mut game_state: ResMut<GameState>,
        game_settings: ResMut<GameSettings>,
    ) {
        // Use try_recv() instead of recv() to avoid blocking the game thread
        if let Ok(mut rx_lock) = arduino_receiver.receiver.lock() {
            while let Ok(action) = rx_lock.try_recv() {
                // Receive Arduino action and perform corresponding operations
                match action {
                    ArduinoAction::MoveRight => {
                        println!("Received MoveRight action");
                        if let Some(Cords(x, y)) = game_state.player.current_position {
                            if y < COLUMNS - 1 && !game_settings.auto_pilot {
                                game_state.player.move_to(Cords(x, y + 1));
                            }
                        }
                    }
                    ArduinoAction::MoveLeft => {
                        println!("Received MoveLeft action");
                        if let Some(Cords(x, y)) = game_state.player.current_position {
                            if y > 0 && !game_settings.auto_pilot {
                                game_state.player.move_to(Cords(x, y - 1));
                            }
                        }
                    }
                    ArduinoAction::Shoot => {
                        println!("Received Shoot action");
                        if let Some(Cords(x, y)) = game_state.player.current_position {
                            // Add a new bullet at the player's position
                            let bullet_position = Cords(x - 1, y);
                            game_state
                                .add_ship(bullet_position, Ship::new_bullet(false, 15))
                                .ok();
                        }
                    }
                }
            }
        }
    }

    pub fn start(&self) {
        let mut game_state = GameState::new();
        let mut game_settings = GameSettings::new();
        let fly_speed = game_settings.set_fly_speed(Default::default(), 4);
        pub fn spawn_flies(
            game_state: &mut GameState,
            game_settings: &GameSettings,
            fly_speed: u32,
            gap: usize,
        ) {

            let number_of_flys = game_settings.number_of_flys;

            for i in 0..COLUMNS {
                let x = if i % 2 == 0 { 0 } else { 1 };
                let y = i;

                game_state
                    .game_board
                    .insert(Cords(x, y), Ship::new_fly(fly_speed, game_settings.fly_move, game_settings.laser_shoot));
            }
        }

        spawn_flies(&mut game_state, &game_settings, fly_speed, 2);

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
            // Insert the Arduino receiver as a proper resource
            .insert_resource(ArduinoReceiver {
                receiver: self.rx.clone()
            })
            .init_state::<GameStateWindows>()
            .add_systems(Update, gameover_button_system)
            .add_systems(Startup, crate::background::background_setup)
            .add_systems(Update, crate::background::move_and_respawn_stars)
            .add_systems(Update, spawn_or_update_settings_display)
            .add_systems(Startup, Self::startup)

            .add_systems(Startup, setup_score_ui)
            .add_systems(Update, update_score_ui)

            //.add_systems(Update, Game::check_and_spawn_flies)

            // Add the system to process Arduino actions
            .add_systems(Update, Self::process_arduino_actions)

            .add_systems(
                Update,
                Self::keyboard_event_system.distributive_run_if(in_state(GameStateWindows::Playing)),
            )
            .add_systems(
                Update,
                Self::gamepad_event_system.distributive_run_if(in_state(GameStateWindows::Playing)),
            )
            .add_systems(
                Update,
                (Self::update).distributive_run_if(in_state(GameStateWindows::Playing)),
            )
            //.add_systems(Update, Game::check_and_spawn_flies)
            .add_systems(OnEnter(GameStateWindows::Menu), Self::menu_enter)
            .add_systems(OnEnter(GameStateWindows::GameOver), game_over_enter)
            .add_systems(OnEnter(GameStateWindows::Playing), |mut commands: Commands| {
                commands.insert_resource(ClearColor(TEXT_COLOR));
            })
            .run();
    }

    pub fn startup(mut commands: Commands, asset_server: Res<AssetServer>) {
        commands.spawn(Camera2dBundle::default());
        commands.insert_resource(ClearColor(Color::BLACK));
        // commands.spawn(AudioPlayer::new(asset_server.load("sounds/galaga.ogg")));
    }
    /*
        pub fn check_and_spawn_flies(
            &self,
            mut commands: Commands,
            mut game_state: ResMut<GameState>,
            game_settings: Res<GameSettings>,
        ) {
            // Check if there are no flies left
            let flies_remaining = game_state
                .game_board
                .values()
                .any(|ship| matches!(ship, Ship::Fly { .. }));

            if !flies_remaining {
                let fly_speed = game_settings.set_fly_speed(Default::default(), 4);
                self.spawn_flies(&mut game_state, &game_settings, fly_speed, 2);
            }
        }
    */
    pub fn update(
        mut commands: Commands,
        mut game_state: ResMut<GameState>,
        mut grid: ResMut<Grid>,
        asset_server: Res<AssetServer>,
        game_settings: Res<GameSettings>,
    ) {
        grid.despawn_entities(&mut commands);

        game_state.ship_actions(&game_settings).unwrap();
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

    pub fn keyboard_event_system(
        mut keyboard_input_events: EventReader<KeyboardInput>,
        mut game_state: ResMut<GameState>,
        mut game_settings: ResMut<GameSettings>,
        mut commands: Commands,
        asset_server: Res<AssetServer>,
    ) {
        if !game_settings.keyboard_enabled {
            return;
        }

        if let Some(Cords(x, y)) = game_state.player.current_position {
            for event in keyboard_input_events.read() {
                if event.state == ButtonState::Pressed {
                    match event.key_code {
                        KeyCode::ArrowLeft => {
                            if y > 0 && !game_settings.auto_pilot {
                                game_state.player.move_to(Cords(x, y - 1));
                            }
                        }

                        KeyCode::ArrowRight => {
                            if y < COLUMNS - 1 && !game_settings.auto_pilot {
                                game_state.player.move_to(Cords(x, y + 1));
                            }
                        }

                        KeyCode::Space => {
                            if !game_settings.auto_shoot {
                                let bullet_position = Cords(x - 1, y);
                                commands.spawn(AudioPlayer::new(asset_server.load("sounds/shooting.ogg")));
                                game_state
                                    .add_ship(bullet_position, Ship::new_bullet(false, 15))
                                    .ok();
                            }
                        }

                        KeyCode::KeyD => {
                            game_settings.invocable = !game_settings.invocable;
                            println!("invincible toggled: {}", game_settings.invocable);
                        }

                        KeyCode::KeyM => {
                            game_settings.auto_pilot = !game_settings.auto_pilot;
                            println!("Auto Pilot toggled: {}", game_settings.auto_pilot);
                        }

                        KeyCode::KeyS => {
                            game_settings.auto_shoot = !game_settings.auto_shoot;
                            println!("Auto Shoot toggled: {}", game_settings.auto_shoot);
                        }

                        KeyCode::KeyA => {
                            game_settings.fly_move = !game_settings.fly_move;
                            println!("Fly Move toggled: {}", game_settings.fly_move);
                        }

                        KeyCode::KeyW => {
                            game_settings.laser_shoot = !game_settings.laser_shoot;
                            println!("Laser Shoot toggled: {}", game_settings.laser_shoot);
                        }

                        _ => {}
                    }
                }
            }
        }

        // Handle auto-move and auto-shoot logic if enabled
        game_settings.handle_auto_move(&mut game_state, &Default::default());
        game_settings.handle_auto_shoot(&mut game_state, &mut commands, &Default::default());
    }

    fn gamepad_event_system(
        mut gamepads: Query<(Entity, &Gamepad)>,
        mut game_state: ResMut<GameState>,
        mut game_settings: ResMut<GameSettings>,
        mut commands: Commands,
        asset_server: Res<AssetServer>,
        mut reset_flag: Local<bool>,
    ) {
        if !game_settings.gamepad_enabled {
            return;
        }

        for (_, gamepad) in &mut gamepads {
            for button in [
                GamepadButton::North,
                GamepadButton::East,
                GamepadButton::South,
                GamepadButton::West,
                GamepadButton::DPadUp,
                GamepadButton::DPadDown,
                GamepadButton::DPadLeft,
                GamepadButton::DPadRight,
            ] {
                if gamepad.just_pressed(button) {
                    match button {
                        GamepadButton::North => {
                            game_settings.invocable = !game_settings.invocable;
                            println!("invincible toggled (Y): {}", game_settings.invocable);
                        }
                        GamepadButton::West => {
                            game_settings.auto_pilot = !game_settings.auto_pilot;
                            println!("Auto Pilot toggled (X): {}", game_settings.auto_pilot);
                        }
                        GamepadButton::South => {
                            game_settings.auto_shoot = !game_settings.auto_shoot;
                            println!("Auto Shoot toggled (A): {}", game_settings.auto_shoot);
                        }
                        GamepadButton::East => {
                            game_settings.fly_move = !game_settings.fly_move;
                            println!("Fly Move toggled (B): {}", game_settings.fly_move);
                        }
                        GamepadButton::DPadRight => {
                            game_settings.laser_shoot = !game_settings.laser_shoot;
                            println!("Laser Shoot toggled (D-pad Right): {}", game_settings.laser_shoot);
                        }
                        _ => {}
                    }
                }
            }

            if gamepad.just_pressed(GamepadButton::LeftTrigger2)
                || gamepad.just_pressed(GamepadButton::RightTrigger2)
            {
                if !game_settings.auto_shoot {
                    if let Some(Cords(x, y)) = game_state.player.current_position {
                        let bullet_position = Cords(x - 1, y);
                        commands.spawn(AudioPlayer::new(asset_server.load("sounds/shooting.ogg")));
                        let _ = game_state.add_ship(bullet_position, Ship::new_bullet(false, 15));
                    }
                }
            }

            if let Some(left_stick_x) = gamepad.get(GamepadAxis::LeftStickX) {
                if let Some(Cords(x, y)) = game_state.player.current_position {
                    if *reset_flag {
                        if left_stick_x.abs() < 0.1 {
                            *reset_flag = false;
                        }
                    } else {
                        if left_stick_x < -0.5 && y > 0 && !game_settings.auto_pilot {
                            game_state.player.move_to(Cords(x, y - 1));
                            *reset_flag = true;
                        } else if left_stick_x > 0.5 && y < COLUMNS - 1 && !game_settings.auto_pilot {
                            game_state.player.move_to(Cords(x, y + 1));
                            *reset_flag = true;
                        }
                    }
                }
            }

            game_settings.handle_auto_move(&mut game_state, &Default::default());
            game_settings.handle_auto_shoot(&mut game_state, &mut commands, &Default::default());
        }
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

}

#[derive(Resource, Debug)]
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