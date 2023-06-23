mod geometry;
mod logic;
mod state;

use state::GameState;

use serde_derive::{Deserialize, Serialize};
use warp::{reply, Filter};

#[derive(Serialize)]
pub struct SnakeInfo {
    pub apiversion: &'static str,
    pub author: &'static str,
    pub color: &'static str,
    pub head: &'static str,
    pub tail: &'static str,
    pub version: &'static str,
}

static THIS_SNAKE: SnakeInfo = SnakeInfo {
    apiversion: "1",
    author: "DanielleB-R",
    color: "#cc00cc",
    head: "default",
    tail: "default",
    version: "0.1.0",
};

#[tokio::main]
async fn main() {
    pretty_env_logger::init();

    let root = warp::get().map(|| reply::json(&THIS_SNAKE));

    let start = warp::post()
        .and(warp::path("start"))
        .and(warp::body::json())
        .map(|body: GameState| {
            logic::start_game(body);
            reply::reply()
        });

    let end = warp::post()
        .and(warp::path("end"))
        .and(warp::body::json())
        .map(|body: GameState| {
            logic::end_game(body);
            reply::reply()
        });

    let move_route = warp::post()
        .and(warp::path("move"))
        .and(warp::body::json())
        .map(|body: GameState| reply::json(&logic::make_move(body)));

    warp::serve(root.or(start).or(end).or(move_route))
        .run(([0, 0, 0, 0], 8080))
        .await
}
