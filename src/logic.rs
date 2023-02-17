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

use core::panic;
use log::info;
use pathfinding::prelude::astar;
use serde_json::{json, Value};

use crate::utils;
use crate::utils::SnakePersonality;
use crate::{Battlesnake, Board as BattlesnakeBoard, Coord as BattlesnakeCoord, Game};
use rust_pathfinding::{Board as PathfindingBoard, Pos};

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

    // main logic

    // 2. avoid directly hitting snakes
    let pathfinding_board = utils::build_pathfinding_board_with_hazards(&personality, board, &you);

    // 3. determine goal
    let result = determine_goal(&personality, &pathfinding_board, board, my_head);
    let moves = if let Some(moves) = result {
        moves
    } else {
        panic!("No moves to make!");
    };

    // 5. MOVE THERE!
    let chosen = determine_next_move(&moves, board, my_head);
    info!("MOVE {}: {}", turn, chosen);
    return json!({ "move": chosen });
}

fn coord_distance(a: &BattlesnakeCoord, b: &BattlesnakeCoord) -> f64 {
    // sqrt((b.0 - b.1)^2 + (a.0 - a.1)^2)
    let b0 = b.x as i32; // DANGER!!!
    let b1 = b.y as i32;
    let a0 = a.x as i32;
    let a1 = a.y as i32;
    f64::sqrt((b0 - a0).pow(2) as f64 + (b1 - a1).pow(2) as f64)
}

fn determine_goal(
    personality: &SnakePersonality,
    pathfinding_board: &PathfindingBoard,
    board: &BattlesnakeBoard,
    head: &BattlesnakeCoord,
) -> Option<(Vec<Pos>, u32)> {
    let self_pos = utils::coord_to_pos(board, head);
    let goal_pos = match personality {
        &SnakePersonality::Snacky => utils::coord_to_pos(board, find_delicious_food(board, head)),
        &SnakePersonality::HeadHunter => {
            utils::coord_to_pos(board, find_delicious_snake(board, head))
        }
        a => panic!("That personality isn't implemented yet: {:?}", a),
    };
    return astar(
        &self_pos,
        |p| {
            pathfinding_board
                .get_successors(p)
                .iter()
                .map(|s| (s.pos, s.cost))
                .collect::<Vec<_>>()
        },
        |p| ((p.0 - goal_pos.0).abs() + (p.1 - goal_pos.1).abs()) as u32,
        |p| *p == goal_pos,
    );
}

fn determine_next_move(
    moves: &(Vec<Pos>, u32),
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

fn find_delicious_snake<'a>(
    board: &'a BattlesnakeBoard,
    head: &BattlesnakeCoord,
) -> &'a BattlesnakeCoord {
    let mut distances = Vec::new();
    let other_snakes = &board.snakes[1..];
    for s in other_snakes {
        let d = coord_distance(head, &s.head);
        let mut smallest_snake_size = 999;
        for s in other_snakes {
            let osd = s.body.len();
            if osd < smallest_snake_size {
                smallest_snake_size = osd;
            }
        }
        distances.push((d, smallest_snake_size));
    }
    println!("{:?}", distances);
    let min = distances
        .iter()
        .filter(|f| f.0 != 0.0)
        .min_by(|a, b| a.partial_cmp(b).unwrap())
        .unwrap();
    println!("{:?}", min);
    let min_clone = min.clone();
    let idx = distances
        .iter()
        .position(|d| d == &min_clone)
        .expect("cant find snake");
    println!("snake idx {}", idx);
    let snake = &other_snakes.get(idx).unwrap();
    println!("{:?}", snake);
    &snake.head
}

fn find_delicious_food<'a>(
    board: &'a BattlesnakeBoard,
    head: &BattlesnakeCoord,
) -> &'a BattlesnakeCoord {
    if board.snakes.len() == 1 {
        let mut distances = Vec::new();
        for f in &board.food {
            let d = coord_distance(head, f);
            distances.push(d);
        }
        let min = distances
            .iter()
            .min_by(|a, b| a.partial_cmp(b).unwrap())
            .unwrap();
        let min_clone = min.clone();
        let idx = distances
            .iter()
            .position(|d| d == &min_clone)
            .expect("cant find food");
        return board.food.get(idx).unwrap();
    } else {
        let mut distances = Vec::new();
        let other_snakes = &board.snakes[1..];
        for f in &board.food {
            let d = coord_distance(head, f);
            let mut closest_other_snake = 999.0;
            for s in other_snakes {
                let osd = coord_distance(&s.head, f);
                if osd < closest_other_snake {
                    closest_other_snake = osd;
                }
            }
            distances.push((d, closest_other_snake));
        }
        println!("{:?}", distances);
        let min = distances
            .iter()
            .min_by(|a, b| a.partial_cmp(b).unwrap())
            .unwrap();
        println!("{:?}", min);
        let min_clone = min.clone();
        let idx = distances
            .iter()
            .position(|d| d == &min_clone)
            .expect("cant find food");
        return board.food.get(idx).unwrap();
    }
}
