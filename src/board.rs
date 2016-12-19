use std::fmt;
use std::collections::BTreeSet;

use rand;
use rand::Rng;

use plane::Plane;
use coordinate::Coordinate;
use orientation::Orientation;
use bombardment_result::BombardmentResult;

#[derive(Clone)]
pub struct Board {
    planes: Vec<Plane>,
    killed_planes: Vec<Plane>,
    hits: Vec<Coordinate>,
    misses: Vec<Coordinate>,
    kills: Vec<Coordinate>,
    empty_indices: BTreeSet<usize>,
    previous_error_message: Option<String>,
}

impl Board {
    pub fn empty_indices(&self) -> &BTreeSet<usize> {
        &self.empty_indices
    }
    pub fn empty_indices_mut(&mut self) -> &mut BTreeSet<usize> {
        &mut self.empty_indices
    }
    pub fn new() -> Board {
        let mut empty_indices: BTreeSet<usize> = BTreeSet::new();
        for i in 0..100 {
            empty_indices.insert(i);
        }
        Board {
            planes: Vec::new(),
            killed_planes: Vec::new(),
            hits: Vec::new(),
            misses: Vec::new(),
            kills: Vec::new(),
            empty_indices: empty_indices,
            previous_error_message: None,
        }
    }
    pub fn get_previous_hit_message(&self) -> String {
        match self.previous_error_message {
            Some(ref msg) => msg.clone(),
            None => "".to_string(),
        }
    }
    pub fn new_random() -> Board {
        let mut temp_board = Board::new();

        let mut rng = rand::thread_rng();
        let mut random_orientations = [0 as usize, 1, 2, 3];
        //TODO: better heuristics, AI levels
        loop {
            for _ in 0..100 {
                let head_candidate = Coordinate::new_random_coordinate();
                let raw_head_candidate = format!("{}", head_candidate);

                rng.shuffle(&mut random_orientations);
                for j in random_orientations.iter() {
                    let orientation_cadidate = Orientation::from(*j as usize);
                    let raw_orientation_candidate = format!("{}", orientation_cadidate);
                    match temp_board.add_new_plane_at(&raw_head_candidate, &raw_orientation_candidate) {
                        Ok(_) => {
                            break;
                        },
                        Err(_msg) => {
                            continue;
                        }
                    }
                }
                if temp_board.planes.len() == 3 {
                    break;
                }
            }
            if temp_board.planes.len() == 3 {
                break;
            } else {
                temp_board.clear_planes();
            }
        }
        temp_board
    }

    pub fn add_new_plane_at(&mut self, head: &str, orientation: &str) -> Result<usize, String> {
        if self.is_in_gameplay() {
            let t = "Cannot add planes mid-game".to_string();
            self.previous_error_message = Some(t.clone());
            return Err(t);
        }
        let t_plane = Plane::new_with_id(head, orientation, self.planes.len()+1);
        match t_plane {
            None => {
                let t = format!("plane cannot spawn at {} in direction {}", head, orientation);
                self.previous_error_message = Some(t.clone());
                Err(t)
            },
            Some(plane) => {
                match plane.is_outside_of_map() {
                    true => {
                        let t = "Plane would fall off the map, try again.".to_string();
                        self.previous_error_message = Some(t.clone());
                        Err(t)
                    },
                    false => {
                        // TODO: return list of overlapping other planes in error message
                        for other in &self.planes {
                            if plane.is_overlapping_with(&other) {
                                let t = format!("Plane would overlap with another one: {}, try again.", other.id());
                                self.previous_error_message = Some(t.clone());
                                return Err(t);
                            }
                        }
                        let head_offset = plane.head().as_usize();
                        self.empty_indices.remove(&head_offset);
                        for tile in plane.tile_iterator() {
                            self.empty_indices.remove(&tile);
                        }
                        self.planes.push(plane);
                        self.previous_error_message = None;
                        Ok(self.planes.last().unwrap().id())
                    },
                }
            }
        }
    }
    fn is_in_gameplay(&self) -> bool {
        !self.hits.is_empty() || !self.misses.is_empty()
    }
    pub fn is_initialized(&self) -> bool {
        self.planes.len() + self.killed_planes.len() == 3
    }
    pub fn hit_at(&mut self, coord: Coordinate) -> BombardmentResult {
        self.empty_indices.remove(&coord.as_usize());
        for i in 0..self.planes.len() {
            if self.planes[i].has_tile(&coord) {
                self.hits.push(coord);
                return BombardmentResult::Hit;
            }
            if self.planes[i].head() == &coord {
                let killed_plane = self.planes.remove(i);
                self.killed_planes.push(killed_plane);
                return BombardmentResult::Kill;
            }
        }
        self.misses.push(coord);
        BombardmentResult::Miss
    }
    pub fn planes(&self) -> &Vec<Plane> {
        &self.planes
    }
    pub fn killed_planes(&self) -> &Vec<Plane> {
        &self.killed_planes
    }
    pub fn killed_planes_mut(&mut self) -> &mut Vec<Plane> {
        &mut self.killed_planes
    }
    pub fn hits(&self) -> &Vec<Coordinate> {
        &self.hits
    }
    pub fn hits_mut(&mut self) -> &mut Vec<Coordinate> {
        &mut self.hits
    }
    pub fn misses(&self) -> &Vec<Coordinate> {
        &self.misses
    }
    pub fn misses_mut(&mut self) -> &mut Vec<Coordinate> {
        &mut self.misses
    }
    pub fn kills(&self) -> &Vec<Coordinate> {
        &self.kills
    }
    pub fn kills_mut(&mut self) -> &mut Vec<Coordinate> {
        &mut self.kills
    }
    pub fn clear_planes(&mut self) {
        self.planes = Vec::new();
    }
    pub fn find_plane_at(&self, at: &Coordinate) -> Option<&Plane> {
        for plane in &self.planes {
            if plane.has_tile(at) {
                return Some(plane);
            }
            if plane.head() == at {
                return Some(plane);
            }
        }
        for plane in &self.killed_planes {
            if plane.has_tile(at) {
                return Some(plane);
            }
            if plane.head() == at {
                return Some(plane);
            }
        }
        None
    }
    pub fn get_plane_by_id(&self, id: usize) -> Option<&Plane> {
        for plane in &self.planes {
            if plane.id() == id {
                return Some(&plane);
            }
        }
        None
    }
}


impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut s = String::new();
        for plane in self.planes() {
            s.push_str(format!("{}{} ", plane.head(), plane.orientation()).as_str());
        }
        write!(f, "{}", s)
    }
}

