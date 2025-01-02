use crate::ship::Ship;
use crate::structs::{Cords, COLUMNS, ROWS, Level, GameLevel};
use std::collections::HashMap;
use std::process::exit;
use std::thread;
use std::time::Duration;
use uuid::Uuid;
use bevy::prelude::*;
use bevy::ecs::system::Resource;

use crate::structs::ShipAction;
use crate::player::Player;

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

        App::new()
            .insert_resource(games_state)
            .insert_resource(Grid::new(50.0))
            .add_plugins(DefaultPlugins)
            .add_systems(Startup, Self::startup)
            .add_systems(Update, Self::update)
            .run();
    }
    fn startup(asset_server: Res<AssetServer>, mut commands: Commands) {
        commands.spawn(AudioPlayer::new(asset_server.load("sounds/galaga.ogg")));
        commands.spawn(Camera2dBundle::default());
    }

    fn update(mut commands: Commands, mut game_state: ResMut<GameState>, mut grid: ResMut<Grid>) {
        // First, despawn all existing entities
        grid.despawn_entities(&mut commands);

        // Perform game state updates
        game_state.ship_actions().unwrap();

        // Add updated entities
        for (&cords, ship) in game_state.game_board.iter() {
            let (r, g, b, a) = ship.display_info();
            let color = Color::rgba_u8(r, g, b, a);
            let position = (cords.0 as usize, cords.1 as usize);
            grid.add_entity(color, position);
        }

        // Render new entities
        grid.render_entities(&mut commands);
    }
}

#[derive(Resource)]
struct Grid {
    cell_size: f32,
    entities: Vec<(Color, (usize, usize))>,
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

    fn add_entity(&mut self, color: Color, position: (usize, usize)) {
        assert!(
            position.0 < ROWS && position.1 < COLUMNS,
            "Position out of bounds!"
        );
        self.entities.push((color, position));
    }

    fn despawn_entities(&mut self, commands: &mut Commands) {
        for entity_id in self.entity_ids.drain(..) {
            commands.entity(entity_id).despawn();
        }
        self.entities.clear();
    }

    fn render_grid(&mut self, commands: &mut Commands) {
        let grid_offset_x = -(COLUMNS as f32 * self.cell_size) / 2.0;
        let grid_offset_y = -(ROWS as f32 * self.cell_size) / 2.0;

        for row in 0..ROWS {
            for col in 0..COLUMNS {
                let x = col as f32 * self.cell_size + grid_offset_x;
                let y = (ROWS - 1 - row) as f32 * self.cell_size + grid_offset_y;

                let entity = commands.spawn((
                    SpriteBundle {
                        transform: Transform::from_translation(Vec3::new(x, y, 0.0)),
                        sprite: Sprite {
                            color: Color::rgb(0.9, 0.9, 0.9),
                            custom_size: Some(Vec2::new(self.cell_size - 2.0, self.cell_size - 2.0)),
                            ..default()
                        },
                        ..default()
                    },
                )).id();

                self.entity_ids.push(entity);
            }
        }
    }

    fn render_entities(&mut self, commands: &mut Commands) {
        let grid_offset_x = -(COLUMNS as f32 * self.cell_size) / 2.0;
        let grid_offset_y = -(ROWS as f32 * self.cell_size) / 2.0;

        for (color, (row, col)) in &self.entities {
            let x = *col as f32 * self.cell_size + grid_offset_x;
            let y = (ROWS - 1 - *row) as f32 * self.cell_size + grid_offset_y;

            let entity = commands.spawn((
                SpriteBundle {
                    transform: Transform::from_translation(Vec3::new(x, y, 1.0)),
                    sprite: Sprite {
                        color: *color,
                        custom_size: Some(Vec2::new(self.cell_size - 4.0, self.cell_size - 4.0)),
                        ..default()
                    },
                    ..default()
                },
            )).id();

            self.entity_ids.push(entity);
        }
    }
}

#[derive(Resource)]
pub struct GameState {
    pub game_board: HashMap<Cords, Ship>,
    pub tick_count: u32,
    pub player: Player,
    pub gamelevel: GameLevel,
}

impl GameState {
    pub fn new() -> GameState {
        let game_level = GameLevel::new(Level::Easy);
        let level_status = game_level.get_level_status();

        GameState {
            game_board: HashMap::new(),
            tick_count: 0,
            player: Player::new(level_status.1),
            gamelevel: game_level,
        }
    }

    pub fn add_ship(&mut self, cords: Cords, ship: Ship) -> Result<(), String> {
        if cords.0 >= ROWS || cords.1 >= COLUMNS {
            return Err(format!("Coordinates are out of bounds."));
        } else if let Some(_existing_ship) = self.remove_ship(cords) {
            self.game_board.insert(cords, Ship::new_explosion());
        } else {
            self.game_board.insert(cords, ship);
        }
        Ok(())
    }

    pub fn remove_ship(&mut self, cords: Cords) -> Option<Ship> {
        self.game_board.remove(&cords)
    }

    pub fn move_ship(&mut self, old_cords: Cords, new_cords: Cords) {
        if let Some(ship) = self.remove_ship(old_cords) {
            self.add_ship(new_cords, ship).ok();
        }
    }

    pub fn ship_actions(&mut self,) -> Result<(), String> {
        let to_update = self
            .game_board
            .iter()
            .map(|(cords, ship)| (*cords, ship.get_id()))
            .collect::<Vec<(Cords, Uuid)>>();

        for (cords, shipid) in to_update {
            if let Some(mut current_ship) = self.game_board.remove(&cords) {
                if current_ship.get_id() != shipid {
                    continue;
                }
                match current_ship.get_action(cords, &mut self.game_board) {
                    ShipAction::Remove => {}
                    ShipAction::Shoot => {
                        let shoot_position = Cords(cords.0 + 1, cords.1);
                        self.add_ship(cords, current_ship)?;
                        self.add_ship(shoot_position, Ship::new_bullet(true))?;
                    }
                    ShipAction::Move(new_cords, wrapped) => {
                        if !wrapped || (wrapped && current_ship.wrap()) {
                            self.add_ship(new_cords, current_ship)?;
                        }
                    }
                    ShipAction::Nothing => self.add_ship(cords, current_ship)?,
                }
            }
        }
        Ok(())
    }

    pub async fn player_actions(&mut self) {
        if let Some(player_pos) = self.player.current_position {
            if self.game_board.get(&player_pos).is_some() {
                self.remove_ship(player_pos);
                self.game_board.insert(player_pos, Ship::new_explosion());

                if let Some(lives) = self.player.handle_collision() {
                    println!("oh crap...lives left: {}", lives - 1);
                } else {
                    println!("ow, you died.");
                    exit(0);
                }
            }
        }

        self.player.respawn(self.game_board.get(&self.player.start_position).is_none());

        if let Some(bullet_pos) = self.player.use_key().await {
            self.add_ship(bullet_pos, Ship::new_bullet(false)).ok();
        }
    }

    pub async fn start_game(&mut self) -> Result<(), String> {
        loop {
            thread::sleep(Duration::from_millis(10));
            self.tick_count += 1;
            self.ship_actions()?;
            self.player_actions().await;
        }
    }
}