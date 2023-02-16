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

use log::info;
use pathfinding::{num_traits::Pow, prelude::bfs};
use serde_json::{json, Value};
use std::{collections::HashMap, path::Path};

use crate::{Battlesnake, Board as BattlesnakeBoard, Coord as BattlesnakeCoord, Game};
use rust_pathfinding::{Board as PathfindingBoard, Pos};

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
    let mut is_move_safe: HashMap<_, _> = vec![
        ("up", true),
        ("down", true),
        ("left", true),
        ("right", true),
    ]
    .into_iter()
    .collect();

    // We've included code to prevent your Battlesnake from moving backwards
    let my_head = &you.body[0]; // Coordinates of your head
    let my_neck = &you.body[1]; // Coordinates of your "neck"

    if my_neck.x < my_head.x {
        // Neck is left of head, don't move left
        is_move_safe.insert("left", false);
    } else if my_neck.x > my_head.x {
        // Neck is right of head, don't move right
        is_move_safe.insert("right", false);
    } else if my_neck.y < my_head.y {
        // Neck is below head, don't move down
        is_move_safe.insert("down", false);
    } else if my_neck.y > my_head.y {
        // Neck is above head, don't move up
        is_move_safe.insert("up", false);
    }

    // TODO: Step 1 - Prevent your Battlesnake from moving out of bounds
    // let board_width = &board.width;
    // let board_height = &board.height;

    // TODO: Step 2 - Prevent your Battlesnake from colliding with itself
    let my_body = &you.body;

    // TODO: Step 3 - Prevent your Battlesnake from colliding with other Battlesnakes
    // let opponents = &board.snakes;

    // Are there any safe moves left?
    let safe_moves = is_move_safe
        .into_iter()
        .filter(|&(_, v)| v)
        .map(|(k, _)| k)
        .collect::<Vec<_>>();

    println!("{:?}", safe_moves);
    let self_pos = coord_to_pos(board, &my_body[0]);
    let goal_pos = coord_to_pos(board, find_delicious_food(board, my_head));
    let pathfinding_board = convert_position_to_pathfinding_board(board, &you);
    let result = bfs(
        &self_pos,
        |p| {
            pathfinding_board
                .get_successors(p)
                .iter()
                .map(|successor| successor.pos)
                .collect::<Vec<_>>()
        },
        |p| *p == goal_pos,
    );
    let result = result.expect("No path found");
    // pathfinding_board.draw_to_image(Path::new(&format!("moves/bfs-{}.png", turn)), Some(&result));
    println!("Current Pathfinder Path: {:?}", result);
    let next_move = result.get(1).expect("No more moves to make");
    let converted_next_move = pos_to_coord(board, next_move);
    println!("Next Pathfinder Move: {:?}", next_move);
    println!("Next Battlesnake Move: {:?}", converted_next_move);
    let chosen = get_next_move_from_coord(&my_body[0], &converted_next_move);

    // TODO: Step 4 - Move towards food instead of random, to regain health and survive longer
    // let food = &board.food;

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

fn find_delicious_food<'a>(
    board: &'a BattlesnakeBoard,
    head: &BattlesnakeCoord,
) -> &'a BattlesnakeCoord {
    if board.snakes.len() == 1 {
        let mut distances = Vec::new();
        for f in &board.food {
            println!("{:?}", f);
            let d = coord_distance(head, f);
            distances.push(d);
        }
        println!("{:?}", distances);
        println!("{:?}", board.food);
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
    }
    board.food.get(0).expect("No food to eat")
}

fn get_next_move_from_coord(me: &BattlesnakeCoord, next: &BattlesnakeCoord) -> &'static str {
    if me.y == next.y {
        if me.x as i32 - next.x as i32 == -1 {
            return "right";
        }
        return "left";
    } else {
        if me.y as i32 - next.y as i32 == -1 {
            return "up";
        }
        return "down";
    }
}

fn convert_position_to_pathfinding_board(
    board: &BattlesnakeBoard,
    me: &Battlesnake,
) -> PathfindingBoard {
    let mut string_board: Vec<String> = Vec::new();
    let me_vec = vec![me];
    let all_snakes = me_vec;
    println!("{} snakes", all_snakes.len());
    println!("{:?}", all_snakes.get(0));
    let snake = all_snakes.get(0).unwrap();
    for row in 0..board.height {
        let mut row_string = "".to_string();
        for col in 0..board.width {
            let mut found_body = false;

            for b in &snake.body {
                let converted = coord_to_pos(board, b);
                let x = converted.0 as u32;
                let y = converted.1 as u32;
                if row == y && col == x {
                    row_string += "X";
                    found_body = true;
                }
            }

            if !found_body {
                row_string += "1";
            }
        }
        string_board.push(row_string);
    }
    for r in &string_board {
        println!("{}", r);
    }
    PathfindingBoard::new(string_board, false)
}

fn coord_to_pos(board: &BattlesnakeBoard, c: &BattlesnakeCoord) -> Pos {
    let (col, row) = (c.x, board.height - 1 - c.y);
    Pos(col as i16, row as i16)
}

fn pos_to_coord(board: &BattlesnakeBoard, c: &Pos) -> BattlesnakeCoord {
    let (col, row) = (c.0 as u32, board.height - 1 - c.1 as u32);
    BattlesnakeCoord { x: col, y: row }
}

#[cfg(test)]
mod tests {

    use rust_pathfinding::Pos as PathfindingPos;

    use crate::{
        logic::{coord_to_pos, pos_to_coord},
        Board as BattlesnakeBoard, Coord as BattlesnakeCoord,
    };

    #[test]
    fn battlesnake_to_pathfinding() {
        let board = BattlesnakeBoard {
            width: 11,
            height: 11,
            food: Vec::new(),
            hazards: Vec::new(),
            snakes: Vec::new(),
        };
        assert_eq!(
            coord_to_pos(&board, &BattlesnakeCoord { x: 0, y: 0 }),
            PathfindingPos(0, 10)
        );
        assert_eq!(
            coord_to_pos(&board, &BattlesnakeCoord { x: 9, y: 0 }),
            PathfindingPos(9, 10)
        );
        assert_eq!(
            coord_to_pos(&board, &BattlesnakeCoord { x: 9, y: 9 }),
            PathfindingPos(9, 1)
        );
        assert_eq!(
            coord_to_pos(&board, &BattlesnakeCoord { x: 0, y: 9 }),
            PathfindingPos(0, 1)
        );
        assert_eq!(
            coord_to_pos(&board, &BattlesnakeCoord { x: 4, y: 4 }),
            PathfindingPos(4, 6)
        );
        assert_eq!(
            coord_to_pos(&board, &BattlesnakeCoord { x: 3, y: 7 }),
            PathfindingPos(3, 3)
        );
        assert_eq!(
            coord_to_pos(&board, &BattlesnakeCoord { x: 3, y: 0 }),
            PathfindingPos(3, 10)
        );
        assert_eq!(
            coord_to_pos(&board, &BattlesnakeCoord { x: 0, y: 7 }),
            PathfindingPos(0, 3)
        );
    }

    #[test]
    fn pathfinding_to_battlesnake() {
        let board = BattlesnakeBoard {
            width: 11,
            height: 11,
            food: Vec::new(),
            hazards: Vec::new(),
            snakes: Vec::new(),
        };
        assert_eq!(
            pos_to_coord(&board, &PathfindingPos(0, 10)),
            BattlesnakeCoord { x: 0, y: 0 }
        );
        assert_eq!(
            pos_to_coord(&board, &PathfindingPos(9, 10)),
            BattlesnakeCoord { x: 9, y: 0 }
        );
        assert_eq!(
            pos_to_coord(&board, &PathfindingPos(9, 1)),
            BattlesnakeCoord { x: 9, y: 9 }
        );
        assert_eq!(
            pos_to_coord(&board, &PathfindingPos(0, 1)),
            BattlesnakeCoord { x: 0, y: 9 }
        );
        assert_eq!(
            pos_to_coord(&board, &PathfindingPos(4, 6)),
            BattlesnakeCoord { x: 4, y: 4 }
        );
        assert_eq!(
            pos_to_coord(&board, &PathfindingPos(3, 3)),
            BattlesnakeCoord { x: 3, y: 7 }
        );
        assert_eq!(
            pos_to_coord(&board, &PathfindingPos(3, 10)),
            BattlesnakeCoord { x: 3, y: 0 }
        );
        assert_eq!(
            pos_to_coord(&board, &PathfindingPos(0, 3)),
            BattlesnakeCoord { x: 0, y: 7 }
        );
    }
}
