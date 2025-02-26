use std::sync::{Arc, Mutex};
use tokio::sync::mpsc;
use tokio::task;
use game::Game;
use game_state::GameState;
use server::start_server;

mod ship;
mod player;
mod game_state;
mod structs;
mod game;
mod settings;
mod fly_patterns;
mod background;
mod game_over;
mod settings_display;
mod score_display;
mod server;

#[derive(Debug)]
enum ArduinoAction {
    MoveRight,
    MoveLeft,
    Shoot,
}

async fn process_message(msg: &str) -> Option<ArduinoAction> {
    let parts: Vec<&str> = msg.split_whitespace().collect();
    if parts.len() != 2 {
        println!("Invalid message format: {}", msg);
        return None;
    }

    let action = parts[0];
    let peak: i32 = parts[1].parse().unwrap_or(0);

    if peak > 900 {
        match action {
            "move_right" => Some(ArduinoAction::MoveRight),
            "move_left" => Some(ArduinoAction::MoveLeft),
            "shoot" => Some(ArduinoAction::Shoot),
            _ => {
                println!("Unknown action: {}", action);
                None
            }
        }
    } else {
        println!("Peak value too low: {}", peak);
        None
    }
}

#[tokio::main]
async fn main() -> Result<(), String> {
    let (tx, rx) = mpsc::channel::<ArduinoAction>(100);

    let rx_arc = Arc::new(Mutex::new(rx));

    let server_handle = tokio::spawn(async move {
        if let Err(e) = start_server(tx).await {
            eprintln!("Server error: {}", e);
        }
    });


    let game_instance = Game {
        rx: rx_arc
    };
    game_instance.start();
    server_handle.await.map_err(|e| e.to_string())?;

    Ok(())
}