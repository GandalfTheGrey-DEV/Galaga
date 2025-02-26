use serde::Deserialize;
use warp::Filter;
use tokio::sync::mpsc::Sender;
use crate::ArduinoAction;

#[derive(Deserialize)]
struct Input {
    peak: i32,
}

pub async fn start_server(tx: Sender<ArduinoAction>) -> Result<(), String> {
    let peakright_route = {
        let tx = tx.clone();
        warp::path("peakright")
            .and(warp::post())
            .and(warp::body::json())
            .map(move |input: Input| {
                if input.peak > 900 {
                    let _ = tx.try_send(ArduinoAction::MoveRight);
                    warp::reply::json(&"Moved right".to_string())
                } else {
                    warp::reply::json(&"Peak too low".to_string())
                }
            })
    };

    let peakleft_route = {
        let tx = tx.clone();
        warp::path("peakleft")
            .and(warp::post())
            .and(warp::body::json())
            .map(move |input: Input| {
                if input.peak > 900 {
                    let _ = tx.try_send(ArduinoAction::MoveLeft);
                    warp::reply::json(&"Moved left".to_string())
                } else {
                    warp::reply::json(&"Peak too low".to_string())
                }
            })
    };

    let peakshoot_route = {
        let tx = tx.clone();
        warp::path("peakshoot")
            .and(warp::post())
            .and(warp::body::json())
            .map(move |input: Input| {
                if input.peak > 900 {
                    let _ = tx.try_send(ArduinoAction::Shoot);
                    warp::reply::json(&"Shooting".to_string())
                } else {
                    warp::reply::json(&"Peak too low".to_string())
                }
            })
    };

    let routes = peakright_route
        .or(peakleft_route)
        .or(peakshoot_route)
        .with(warp::cors().allow_any_origin());

    warp::serve(routes).run(([0, 0, 0, 0], 3030)).await;
    Ok(())
}
