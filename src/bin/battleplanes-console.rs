extern crate battleplanes;

use std::io;
use std::fmt;
use std::io::Write;

fn main() {
    let mut board = ConsoleBoard(battleplanes::Board::new());
    loop {
        println!("{}", board);
        match board.planes().len() {
            3 => {
                let new_hit = read_line_with_prompt("Bombard coordinate: ");
                if board.hit_at(new_hit.as_str()) {
                    println!("You've hit at {}", new_hit);
                } else {
                    println!("You've missed at {}", new_hit);
                }
            },
            _ => {
                let new_head = read_line_with_prompt("new plane at: ");
                let new_orientation = read_line_with_prompt("orientation: ");
                match board.add_new_plane_at(new_head.as_str(), new_orientation.as_str()) {
                    Ok(_) => {},
                    Err(msg) => {
                        println!("{}", msg);
                        continue
                    }
                };
            },
        }
    }
}

fn read_line_with_prompt(prompt: &str) -> String {
    let mut value = String::new();
    loop {
        print!("{}", prompt);
        io::stdout().flush();
        let read_count = io::stdin().read_line(&mut value).expect(prompt);
        if read_count > 1 {
            break;
        }
    }
    value.trim().to_string()
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
    fn hit_at(&mut self, tile: &str) -> bool {
        self.0.hit_at(tile)
    }
    fn add_new_plane_at(&mut self, head: &str, orientation: &str) -> Result<&battleplanes::Plane, String> {
        self.0.add_new_plane_at(head, orientation)
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
            let (head_x, head_y) = plane.head().as_usize();
            let head_offset = offset_heuristic(head_x, head_y);
            byte_grid[head_offset] = head_c;

            for tile in plane.tile_iterator().filter_map(|t| t) {
                let (tile_x, tile_y) = tile.as_usize();
                let tile_offset = offset_heuristic(tile_x, tile_y);
                byte_grid[tile_offset] = 'o' as u8;
            }
        }
        for hit in self.hits() {
            let (hit_x, hit_y) = hit.as_usize();
            let hit_offset = offset_heuristic(hit_x, hit_y);
            byte_grid[hit_offset] = 'x' as u8;
        }
        for miss in self.misses() {
            let (miss_x, miss_y) = miss.as_usize();
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
