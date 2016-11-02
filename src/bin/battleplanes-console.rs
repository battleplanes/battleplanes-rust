extern crate battleplanes;

use std::io;
use std::fmt;
use std::io::Write;

fn main() {
    let mut game = battleplanes::Game::new_random_starter();
    let ai_board = battleplanes::Board::new_random();
    println!("AI boards");
    println!("{}", player_boards_as_string(&ai_board, &game.scrapbook_opponent));
    loop {
        match game.gameplay {
            battleplanes::GamePlay::YouPlaceNewPlane => {
                println!("Your boards");
                println!("{}", player_boards_as_string(&game.board_you, &game.scrapbook_you));
                let new_head = read_line_with_prompt("new plane at: ");
                let new_orientation = read_line_with_prompt("orientation: ");
                match game.board_you.add_new_plane_at(new_head.as_str(), new_orientation.as_str()) {
                    Ok(_) => {
                        game.next_logical_state();
                    },
                    Err(msg) => {
                        println!("{}", msg);
                        continue
                    }
                };
            },
            battleplanes::GamePlay::OpponentPlacesNewPlane => {
                let current_index = &game.board_opponent.planes().len();
                let new_plane = &ai_board.planes()[*current_index];
                let new_head = format!("{}", new_plane.head());
                let new_orientation = format!("{}", new_plane.orientation());
                match game.board_opponent.add_new_plane_at(new_head.as_str(), new_orientation.as_str()) {
                    Ok(_) => {
                        println!("AI boards");
                        println!("{}", player_boards_as_string(&game.board_opponent, &game.scrapbook_opponent));
                        game.next_logical_state();
                    },
                    Err(msg) => {
                        println!("{}", msg);
                        continue
                    }
                };
            },
            battleplanes::GamePlay::YouBombard => {
                //dev mode
                println!("AI boards");
                println!("{}", player_boards_as_string(&game.board_opponent, &game.scrapbook_opponent));
                //end dev mode
                println!("Your boards");
                println!("{}", player_boards_as_string(&game.board_you, &game.scrapbook_you));
                let new_hit = read_line_with_prompt("Bombard coordinate: ");
                match game.you_hit_at(new_hit.as_str()) {
                    battleplanes::BombardmentResult::Hit => {
                        println!("You've hit at {}", new_hit);
                        game.next_logical_state();
                    },
                    battleplanes::BombardmentResult::Miss => {
                        println!("You've missed at {}", new_hit);
                        game.next_logical_state();
                    },
                    battleplanes::BombardmentResult::Kill => {
                        println!("You've killed at {}", new_hit);
                        game.next_logical_state();
                    },
                    battleplanes::BombardmentResult::Retry => {
                        println!("Retry");
                    },
                };
            },
            battleplanes::GamePlay::OpponentBombards => {
                let (result, tile) = game.opponent_hits_randomly();
                match result {
                    battleplanes::BombardmentResult::Hit => {
                        println!("AI hits at {}", tile.unwrap())
                    },
                    battleplanes::BombardmentResult::Miss => {
                        println!("AI misses at {}", tile.unwrap())
                    },
                    battleplanes::BombardmentResult::Kill => {
                        println!("AI kills your plane at {}", tile.unwrap())
                    },
                    _ => { },
                };
                game.next_logical_state();
            },
            battleplanes::GamePlay::YouWon => {
                println!("Congratulations, you have won the game!");
                break;
            },
            battleplanes::GamePlay::OpponentWon => {
                println!("Unfortunately, the AI defeated you");
                break;
            },
        };
    }
}

fn read_line_with_prompt(prompt: &str) -> String {
    let mut value = String::new();
    loop {
        print!("{}", prompt);
        match io::stdout().flush() {
            Ok(_) => {},
            Err(_) => {},
        };
        let read_count = io::stdin().read_line(&mut value).expect(prompt);
        if read_count > 1 {
            break;
        }
    }
    value.trim().to_string()
}

fn player_boards_as_string(left: &battleplanes::Board, right: &battleplanes::Board) -> String {
    let template = format!("
  ABCDEFGHIJ     ABCDEFGHIJ  
 1          1   1          1 
 2          2   2          2 
 3          3   3          3 
 4          4   4          4 
 5          5   5          5 
 6          6   6          6 
 7          7   7          7 
 8          8   8          8 
 9          9   9          9 
10          10 10          10
  ABCDEFGHIJ     ABCDEFGHIJ  
");
    let mut board_data = template.into_bytes();
    let board_heuristic_left = |x:usize, y:usize| 33 + y*30 + x;
    let board_heuristic_right = |x:usize, y:usize| 48 + y*30 + x;
    draw_planes_onto_data(&mut board_data, &left, board_heuristic_left);
    draw_planes_onto_data(&mut board_data, &right, board_heuristic_right);
    String::from_utf8(board_data).unwrap()
}

fn draw_planes_onto_data<F>(byte_grid: &mut Vec<u8>, board: &battleplanes::Board, offset_heuristic: F)
    where F: Fn(usize, usize) -> usize
{
    for plane in board.planes() {
        let head_c : u8 = match plane.orientation() {
            &battleplanes::Orientation::North => '^' as u8,
            &battleplanes::Orientation::South => 'v' as u8,
            &battleplanes::Orientation::West => '<' as u8,
            &battleplanes::Orientation::East => '>' as u8,
        };
        let (head_x, head_y) = plane.head().as_tuple();
        let head_offset = offset_heuristic(head_x, head_y);
        byte_grid[head_offset] = head_c;

        for tile in plane.coordinate_iterator().filter_map(|t| t) {
            let (tile_x, tile_y) = tile.as_tuple();
            let tile_offset = offset_heuristic(tile_x, tile_y);
            byte_grid[tile_offset] = 'o' as u8;
        }
    }
    for hit in board.hits() {
        let (hit_x, hit_y) = hit.as_tuple();
        let hit_offset = offset_heuristic(hit_x, hit_y);
        byte_grid[hit_offset] = 'x' as u8;
    }
    for miss in board.misses() {
        let (miss_x, miss_y) = miss.as_tuple();
        let miss_offset = offset_heuristic(miss_x, miss_y);
        byte_grid[miss_offset] = '*' as u8;
    }
    for killed in board.killed_planes() {
        let (killed_x, killed_y) = killed.head().as_tuple();
        let killed_offset = offset_heuristic(killed_x, killed_y);
        byte_grid[killed_offset] = 'X' as u8;
    }
    for kill in board.kills() {
        let (kill_x, kill_y) = kill.as_tuple();
        let kill_offset = offset_heuristic(kill_x, kill_y);
        byte_grid[kill_offset] = 'X' as u8;
    }
}

struct ConsoleBoard<T>(pub T);

impl ConsoleBoard<battleplanes::Board> {
    pub fn planes(&self) -> &Vec<battleplanes::Plane> {
        &self.0.planes()
    }
    pub fn hits(&self) -> &Vec<battleplanes::Coordinate> {
        self.0.hits()
    }
    pub fn misses(&self) -> &Vec<battleplanes::Coordinate> {
        self.0.misses()
    }
}

impl fmt::Display for ConsoleBoard<battleplanes::Board> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let ascii_board = format!("
  ABCDEFGHIJ
 1          1 
 2          2 
 3          3 
 4          4 
 5          5 
 6          6 
 7          7 
 8          8 
 9          9 
10          10
  ABCDEFGHIJ
");
        let mut byte_grid = ascii_board.into_bytes();
        let offset_heuristic = |x:usize, y:usize| 16 + y*15 + x;
        for plane in self.planes() {
            let head_c : u8 = match plane.orientation() {
                &battleplanes::Orientation::North => '^' as u8,
                &battleplanes::Orientation::South => 'v' as u8,
                &battleplanes::Orientation::West => '<' as u8,
                &battleplanes::Orientation::East => '>' as u8,
            };
            let (head_x, head_y) = plane.head().as_tuple();
            let head_offset = offset_heuristic(head_x, head_y);
            byte_grid[head_offset] = head_c;

            for tile in plane.coordinate_iterator().filter_map(|t| t) {
                let (tile_x, tile_y) = tile.as_tuple();
                let tile_offset = offset_heuristic(tile_x, tile_y);
                byte_grid[tile_offset] = 'o' as u8;
            }
        }
        for hit in self.hits() {
            let (hit_x, hit_y) = hit.as_tuple();
            let hit_offset = offset_heuristic(hit_x, hit_y);
            byte_grid[hit_offset] = 'x' as u8;
        }
        for miss in self.misses() {
            let (miss_x, miss_y) = miss.as_tuple();
            let miss_offset = offset_heuristic(miss_x, miss_y);
            byte_grid[miss_offset] = '*' as u8;
        }
        let displayed = String::from_utf8(byte_grid).unwrap();
        write!(f, "{}", displayed)
    }
}

struct ConsolePlane<T>(pub T);

impl ConsolePlane<battleplanes::Plane> {
    fn head(&self) -> &battleplanes::Coordinate {
        self.0.head()
    }
    fn orientation(&self) -> &battleplanes::Orientation {
        self.0.orientation()
    }
}

impl fmt::Display for ConsolePlane<battleplanes::Plane> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} {}", self.head(), self.orientation())
    }
}
