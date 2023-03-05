use crate::{Battlesnake, Board as BattlesnakeBoard, Coord as BattlesnakeCoord};
use itertools::Itertools;
use rust_pathfinding::{Board as PathfindingBoard, PathfindingPos};

use crate::logic::{SnakeMode, SnakePersonality};

pub fn get_next_move_from_coord(me: &BattlesnakeCoord, next: &BattlesnakeCoord) -> &'static str {
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

pub fn coord_distance(a: &BattlesnakeCoord, b: &BattlesnakeCoord) -> f64 {
    let b0 = b.x as i32;
    let b1 = b.y as i32;
    let a0 = a.x as i32;
    let a1 = a.y as i32;
    f64::sqrt((b0 - a0).pow(2) as f64 + (b1 - a1).pow(2) as f64)
}

pub fn get_target_body_from_personality<'a>(
    snake: &'a Battlesnake,
    personality: &SnakePersonality,
) -> Vec<&'a BattlesnakeCoord> {
    let avoid = match personality {
        SnakePersonality::HeadHunter => {
            let body = &snake.body[1..];
            let unique_body = body
                .iter()
                .unique_by(|f| format!("{}_{}", f.x, f.y))
                .collect();
            unique_body
        }
        _ => {
            let a = snake.body.iter().map(|f| f).collect();
            return a;
        }
    };
    avoid
}

pub fn get_snake_mode(
    board: &BattlesnakeBoard,
    snake: &Battlesnake,
    personality: &SnakePersonality,
) -> SnakeMode {
    match personality {
        &SnakePersonality::HeadHunter => {
            let mut largest_snake = 0;
            for s in &board.snakes[1..] {
                if s.body.len() > largest_snake {
                    largest_snake = s.body.len();
                }
            }
            println!(
                "Our Size: {}, Largest Snake Size: {}",
                snake.body.len(),
                largest_snake
            );
            if snake.body.len() > largest_snake {
                SnakeMode::Kill
            } else {
                SnakeMode::Eat
            }
        }
        _ => SnakeMode::Eat,
    }
}

pub fn build_pathfinding_board_with_hazards(
    personality: &SnakePersonality,
    board: &BattlesnakeBoard,
    _me: &Battlesnake,
) -> (PathfindingBoard, Vec<String>) {
    let mut string_board: Vec<String> = Vec::new();
    let all_snakes = &board.snakes;
    for row in 0..board.height {
        let mut row_string = "".to_string();
        for col in 0..board.width {
            let mut found_body = false;
            for snake in all_snakes {
                if !found_body {
                    let avoid = get_target_body_from_personality(snake, personality);
                    for b in avoid.iter() {
                        let converted = coord_to_pos(board, b);
                        let x = converted.0 as u32;
                        let y = converted.1 as u32;
                        if row == y && col == x {
                            row_string += "X";
                            found_body = true;
                        }
                    }
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
    let copy_of_board = string_board.clone();
    let pathfinding_board = PathfindingBoard::new(string_board, false);
    (pathfinding_board, copy_of_board)
}

pub fn coord_to_pos(board: &BattlesnakeBoard, c: &BattlesnakeCoord) -> PathfindingPos {
    let (col, row) = (c.x, board.height - 1 - c.y);
    PathfindingPos(col as i16, row as i16)
}

pub fn pos_to_coord(board: &BattlesnakeBoard, c: &PathfindingPos) -> BattlesnakeCoord {
    let (col, row) = (c.0 as u32, board.height - 1 - c.1 as u32);
    BattlesnakeCoord { x: col, y: row }
}

#[cfg(test)]
mod tests {

    use rust_pathfinding::PathfindingPos;

    use crate::{
        utils::{build_pathfinding_board_with_hazards, coord_to_pos, pos_to_coord},
        Battlesnake, Board as BattlesnakeBoard, Coord as BattlesnakeCoord,
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

    #[test]
    fn headhunter_snake() {
        let snakes = vec![
            Battlesnake {
                id: "gs_bcpvggkD9kYTTcmRGHRCCGXD".to_string(),
                name: "Frostflayer".to_string(),
                health: 100,
                body: vec![
                    BattlesnakeCoord { x: 2, y: 6 },
                    BattlesnakeCoord { x: 1, y: 6 },
                    BattlesnakeCoord { x: 1, y: 5 },
                ],
                head: BattlesnakeCoord { x: 2, y: 6 },
                length: 3,
                latency: "".to_string(),
                shout: None,
            },
            Battlesnake {
                id: "gs_bcpvggkD9kYTTcmRGHRCCGXD".to_string(),
                name: "Scared Bot".to_string(),
                health: 98,
                body: vec![
                    BattlesnakeCoord { x: 9, y: 4 },
                    BattlesnakeCoord { x: 9, y: 5 },
                    BattlesnakeCoord { x: 8, y: 5 },
                ],
                head: BattlesnakeCoord { x: 9, y: 4 },
                length: 3,
                latency: "1".to_string(),
                shout: Some("".to_string()),
            },
        ];
        let board = BattlesnakeBoard {
            width: 11,
            height: 11,
            food: Vec::new(),
            hazards: Vec::new(),
            snakes: snakes,
        };
        let (_, board_string) = build_pathfinding_board_with_hazards(
            &crate::utils::SnakePersonality::HeadHunter,
            &board,
            &board.snakes[0],
        );

        let expected_board = vec![
            "11111111111",
            "11111111111",
            "11111111111",
            "11111111111",
            "1X111111111",
            "1X111111XX1",
            "11111111111",
            "11111111111",
            "11111111111",
            "11111111111",
            "11111111111",
        ];

        assert_eq!(board_string, expected_board);
    }

    #[test]
    fn headhunter_snake_duplicates() {
        let snakes = vec![
            Battlesnake {
                id: "gs_bcpvggkD9kYTTcmRGHRCCGXD".to_string(),
                name: "Frostflayer".to_string(),
                health: 100,
                body: vec![
                    BattlesnakeCoord { x: 2, y: 6 },
                    BattlesnakeCoord { x: 1, y: 6 },
                    BattlesnakeCoord { x: 1, y: 6 },
                    BattlesnakeCoord { x: 1, y: 5 },
                ],
                head: BattlesnakeCoord { x: 2, y: 6 },
                length: 3,
                latency: "".to_string(),
                shout: None,
            },
            Battlesnake {
                id: "gs_bcpvggkD9kYTTcmRGHRCCGXD".to_string(),
                name: "Scared Bot".to_string(),
                health: 98,
                body: vec![
                    BattlesnakeCoord { x: 9, y: 4 },
                    BattlesnakeCoord { x: 9, y: 5 },
                    BattlesnakeCoord { x: 9, y: 5 },
                    BattlesnakeCoord { x: 8, y: 5 },
                ],
                head: BattlesnakeCoord { x: 9, y: 4 },
                length: 3,
                latency: "1".to_string(),
                shout: Some("".to_string()),
            },
        ];
        let board = BattlesnakeBoard {
            width: 11,
            height: 11,
            food: Vec::new(),
            hazards: Vec::new(),
            snakes: snakes,
        };
        let (_, board_string) = build_pathfinding_board_with_hazards(
            &crate::utils::SnakePersonality::HeadHunter,
            &board,
            &board.snakes[0],
        );

        let expected_board = vec![
            "11111111111",
            "11111111111",
            "11111111111",
            "11111111111",
            "1X111111111",
            "1X111111XX1",
            "11111111111",
            "11111111111",
            "11111111111",
            "11111111111",
            "11111111111",
        ];

        assert_eq!(board_string, expected_board);
    }
}
