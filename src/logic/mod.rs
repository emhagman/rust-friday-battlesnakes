// Welcome to
// __________         __    __  .__                               __
// \______   \_____ _/  |__/  |_|  |   ____   ______ ____ _____  |  | __ ____
//  |    |  _/\__  \\   __\   __\  | _/ __ \ /  ___//    \\__  \ |  |/ // __ \
//  |    |   \ / __ \|  |  |  | |  |_\  ___/ \___ \|   |  \/ __ \|    <\  ___/
//  |________/(______/__|  |__| |____/\_____>______>___|__(______/__|__\\_____>
//
// This file can be a nice home for your Battlesnake logic and helper functions.
//
// To get you started we've included code to prevent your Battlesnake from moving backwards.
// For more info see docs.battlesnake.com

mod goal;

use core::panic;
use log::info;
use serde_json::{json, Value};
use std::sync::Arc;

use crate::learning::MyState;
use crate::utils::{self};
use crate::AGENT_TRAINER;
use crate::{Battlesnake, Board as BattlesnakeBoard, Coord as BattlesnakeCoord, Game};
use rust_pathfinding::PathfindingPos;

#[derive(Debug)]
pub enum SnakePersonality {
    Hungry,     // Eats food no matter what
    Timid,      // Avoid snakes at all costs
    HeadHunter, // Kills other snakes
    Snacky,     // Eats food when it's safe to do so
}

#[derive(Debug, PartialEq)]
pub enum SnakeMode {
    Eat,
    Kill,
}

// Logic Loop
// 1. Choose a personality (Down The Road)
// 2. Find enemy bodies on the board and "avoid"
// Sub-logic loop
//  3a. (Avoid other objects based on personality)
//  3b. Determine goal (based on personality)
//  4. ADAPT
//      - check behavior of other snakes
//      - ????

// info is called when you create your Battlesnake on play.battlesnake.com
// and controls your Battlesnake's appearance
// TIP: If you open your Battlesnake URL in a browser you should see this data
pub fn info() -> Value {
    info!("INFO");
    return json!({
        "apiversion": "1",
        "author": "", // TODO: Your Battlesnake Username
        "color": "#888888", // TODO: Choose color
        "head": "default", // TODO: Choose head
        "tail": "default", // TODO: Choose tail
    });
}

// start is called when your Battlesnake begins a game
pub fn start(_game: &Game, _turn: &u32, _board: &BattlesnakeBoard, _you: &Battlesnake) {
    info!("GAME START");
    let trainer = Arc::clone(&AGENT_TRAINER);
    let trainer_lock = trainer.lock().unwrap();
    let action = trainer_lock.best_action(&MyState {
        x: 0,
        y: 0,
        goal: (5, 5),
    });
    info!("BEST ACTION {:?}", action);
}

// end is called when your Battlesnake finishes a game
pub fn end(_game: &Game, _turn: &u32, _board: &BattlesnakeBoard, _you: &Battlesnake) {
    info!("GAME OVER");
}

// move is called on every turn and returns your next move
// Valid moves are "up", "down", "left", or "right"
// See https://docs.battlesnake.com/api/example-move for available data
pub fn get_move(_game: &Game, turn: &u32, board: &BattlesnakeBoard, you: &Battlesnake) -> Value {
    let my_head = &you.body[0]; // Coordinates of your head
    let personality = SnakePersonality::HeadHunter;

    // WHAT MODE AM I IN?????
    let mode = utils::get_snake_mode(board, you, &personality);
    println!("Snake Mode: {:?}", mode);

    // main logic

    // 2. avoid directly hitting snakes
    let (pathfinding_board, _) =
        utils::build_pathfinding_board_with_hazards(&personality, board, &you);

    // 3. determine goal
    let result = goal::determine_goal(&personality, &mode, &pathfinding_board, board, my_head);
    let moves = if let Some(moves) = result {
        moves
    } else {
        panic!("No moves to make!");
    };

    // 4. ?

    // 5. MOVE THERE!
    let chosen = determine_next_move(&moves, board, my_head);
    info!("MOVE {}: {}", turn, chosen);
    return json!({ "move": chosen });
}

fn determine_next_move(
    moves: &(Vec<PathfindingPos>, u32),
    board: &BattlesnakeBoard,
    head: &BattlesnakeCoord,
) -> &'static str {
    println!("Current Pathfinder Path: {:?}", moves);
    let next_move = moves.0.get(1).expect("No more moves to make");
    let converted_next_move = utils::pos_to_coord(board, next_move);
    println!("Next Pathfinder Move: {:?}", next_move);
    println!("Next Battlesnake Move: {:?}", converted_next_move);
    utils::get_next_move_from_coord(head, &converted_next_move)
}
