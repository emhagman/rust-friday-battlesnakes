use crate::{Battlesnake, Board as BattlesnakeBoard, Coord as BattlesnakeCoord};
use rust_pathfinding::{Board as PathfindingBoard, Pos};

#[derive(Debug)]
pub enum SnakePersonality {
    Hungry,     // Eats food no matter what
    Timid,      // Avoid snakes at all costs
    HeadHunter, // Kills other snakes
    Snacky,     // Eats food when it's safe to do so
}

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

pub fn build_pathfinding_board_with_hazards(
    personality: &SnakePersonality,
    board: &BattlesnakeBoard,
    me: &Battlesnake,
) -> PathfindingBoard {
    let mut string_board: Vec<String> = Vec::new();
    let all_snakes = &board.snakes;
    for row in 0..board.height {
        let mut row_string = "".to_string();
        for col in 0..board.width {
            let mut found_body = false;
            for snake in all_snakes {
                let avoid = match personality {
                    SnakePersonality::HeadHunter => &snake.body[1..],
                    _ => &snake.body,
                };
                if !found_body {
                    for b in avoid {
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
    PathfindingBoard::new(string_board, false)
}

pub fn coord_to_pos(board: &BattlesnakeBoard, c: &BattlesnakeCoord) -> Pos {
    let (col, row) = (c.x, board.height - 1 - c.y);
    Pos(col as i16, row as i16)
}

pub fn pos_to_coord(board: &BattlesnakeBoard, c: &Pos) -> BattlesnakeCoord {
    let (col, row) = (c.0 as u32, board.height - 1 - c.1 as u32);
    BattlesnakeCoord { x: col, y: row }
}

#[cfg(test)]
mod tests {

    use rust_pathfinding::Pos as PathfindingPos;

    use crate::{
        utils::{coord_to_pos, pos_to_coord},
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
