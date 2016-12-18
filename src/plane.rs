use coordinate::Coordinate;
use orientation::Orientation;

#[derive(Clone)]
pub struct PlanePositionIterator<'a> {
    head: &'a Coordinate,
    orientation: &'a Orientation,
    current_tile: usize,
}

impl<'a> Iterator for PlanePositionIterator<'a> {
    type Item = Option<Coordinate>;
    fn next(&mut self) -> Option<Option<Coordinate>> {
        use Orientation::*;
        if self.current_tile > 8 {
            return None;
        }
        let tiles = match self.orientation {
            &North => vec![(-2,  1), (-1,  1), ( 0,  1), ( 1,  1), ( 2,  1), ( 0,  2), (-1,  3), ( 0,  3), ( 1,  3)],
            &South => vec![( 2, -1), ( 1, -1), ( 0, -1), (-1, -1), (-2, -1), ( 0, -2), ( 1, -3), ( 0, -3), (-1, -3)],
            &East =>  vec![(-1, -2), (-1, -1), (-1,  0), (-1,  1), (-1,  2), (-2,  0), (-3, -1), (-3,  0), (-3,  1)],
            &West =>  vec![( 1,  2), ( 1,  1), ( 1,  0), ( 1, -1), ( 1, -2), ( 2,  0), ( 3,  1), ( 3,  0), ( 3, -1)],
        };
        let move_x = tiles[self.current_tile].0;
        let move_y = tiles[self.current_tile].1;
        self.current_tile += 1;
        let t = self.head.new_moved_by(move_x, move_y);
        Some(t)
    }
}
pub struct TileIterator<'a> {
    coordinate_iterator: PlanePositionIterator<'a>,
}

impl<'a> Iterator for TileIterator<'a> {
    type Item = usize;
    fn next(&mut self) -> Option<usize> {
        loop {
            match self.coordinate_iterator.next() {
                Some(maybe_coord) => {
                    match maybe_coord {
                        Some(coord) => {
                            return Some(coord.as_usize());
                        },
                        None => continue,
                    }
                },
                None => return None,
            }
        };
    }
}

#[derive(Clone)]
pub struct Plane {
    head: Coordinate,
    orientation: Orientation,
    id: usize,
}

impl Plane {
    pub fn new(from: &str, orientation: &str) -> Option<Plane> {
        Plane::new_with_id(from, orientation, 0)
    }
    pub fn new_with_id(from: &str, orientation: &str, id: usize) -> Option<Plane> {
        match (Coordinate::new(from), Orientation::new(orientation)) {
            (Some(c), Some(o)) => Some(Plane {
                head: c,
                orientation: o,
                id: id,
            }),
            _ => None,
        }
    }
    pub fn coordinate_iterator(&self) -> PlanePositionIterator {
        PlanePositionIterator {
            head: &self.head,
            orientation: &self.orientation,
            current_tile: 0,
        }
    }
    pub fn tile_iterator(&self) -> TileIterator {
        TileIterator {
            coordinate_iterator: self.coordinate_iterator(),
        }
    }
    pub fn orientation(&self) -> &Orientation {
        &self.orientation
    }
    pub fn head(&self) -> &Coordinate {
        &self.head
    }
    pub fn is_outside_of_map(&self) -> bool {
        self.coordinate_iterator().find(|x: &Option<Coordinate>| *x == None) == Some(None)
    }
    pub fn is_overlapping_with(&self, other: &Plane) -> bool {
        if self.head == other.head {
            return true;
        }
        for tile in self.coordinate_iterator().filter_map(|t| t) {
            if tile == other.head {
                return true;
            }
            for other_tile in other.coordinate_iterator().filter_map(|t| t) {
                if tile == other_tile {
                    return true;
                }
                if self.head == other_tile {
                    return true;
                }
            }
        }
        false
    }
    pub fn has_tile(&self, needle: &Coordinate) -> bool {
        for coord in self.coordinate_iterator().filter_map(|t| t) {
            if *needle == coord {
                return true;
            }
        }
        return false;
    }
    pub fn id(&self) -> usize {
        self.id
    }
}

