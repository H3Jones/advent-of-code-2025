use std::collections::HashSet;

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
struct Position {
    x: u32,
    y: u32,
}

impl Position {
    fn from_str(position_str: &str) -> Self {
        let parts: Vec<&str> = position_str.split(',').collect();
        let values: Vec<u32> = parts
            .iter()
            .map(|s| s.parse::<u32>().expect("Invalid number"))
            .collect();
        Self {
            x: values[0],
            y: values[1],
        }
    }

    //
    fn find_area(&self, other: &Position) -> i64 {
        let dx = self.x.abs_diff(other.x) as i64;
        let dy = self.y.abs_diff(other.y) as i64;

        if dx == 0 {
            return dy + 1;
        };

        if dy == 0 {
            return dx + 1;
        };

        (dx + 1) * (dy + 1)
    }

    //
    fn tile_to_inclusive(&self, other: &Position) -> Vec<Position> {
        let dx = self.x.abs_diff(other.x) as i64;
        let dy = self.y.abs_diff(other.y) as i64;

        let mut output: Vec<Position> = Vec::new();
        //Same column
        if dx == 0 {
            let x = &self.x;

            // If reversed then we need to reverse the output
            let reversed = self.y > other.y;

            let y_start = self.y.min(other.y);
            let y_end = self.y.max(other.y);

            for y in y_start..=y_end {
                output.push(Position { x: *x, y: y });
            }

            if reversed {
                output.reverse();
            };
        };
        //Same Row
        if dy == 0 {
            let y = &self.y;

            // If reversed then we need to reverse the output
            let reversed = self.x > other.x;

            let x_start = self.x.min(other.x);
            let x_end = self.x.max(other.x);

            for x in x_start..=x_end {
                output.push(Position { x: x, y: *y });
            }

            if reversed {
                output.reverse();
            };
        };
        output
    }
}

#[derive(Debug)]
struct Rectangle {
    position1: Position,
    position2: Position,
    area: i64,
}

impl Rectangle {
    fn get_positions(&self, corners_only: bool) -> Vec<Position> {
        let min_x = self.position1.x.min(self.position2.x);
        let max_x = self.position1.x.max(self.position2.x);
        let min_y = self.position1.y.min(self.position2.y);
        let max_y = self.position1.y.max(self.position2.y);

        let mut points: Vec<Position> = Vec::new();

        if corners_only {
            points.push(Position { x: min_x, y: min_y });
            points.push(Position { x: max_x, y: min_y });
            points.push(Position { x: min_x, y: max_y });
            points.push(Position { x: max_x, y: max_y });

            return points;
        } else {
            //Top Edge (min_y): x goes from min_x to max_x
            for x in min_x..=max_x {
                points.push(Position { x, y: min_y });
            }

            // Bottom Edge (max_y): x goes from min_x to max_x
            for x in min_x..=max_x {
                points.push(Position { x, y: max_y });
            }

            // Left Edge (min_x): y goes from min_y to max_y
            for y in min_y..=max_y {
                points.push(Position { x: min_x, y });
            }

            //Right Edge (max_x): y goes from min_y to max_y
            for y in min_y..=max_y {
                points.push(Position { x: max_x, y });
            }

            //Remove duplicated
            let unique_points: HashSet<Position> = points.into_iter().collect();
            return unique_points.into_iter().collect();
        }
    }
}

fn fill_in_tiles(red_tiles: &Vec<Position>) -> HashSet<Position> {
    let mut set: HashSet<Position> = HashSet::new();
    for (idx, _tile) in red_tiles.iter().enumerate().skip(1) {
        let tile1 = &red_tiles[idx - 1];
        let tile2 = &red_tiles[idx];

        let filled_in = tile1.tile_to_inclusive(tile2);
        for tile in filled_in.iter() {
            set.insert(tile.clone());
        }
    }
    // last tile wraps round
    let last_tile = &red_tiles.last().unwrap();
    let first_tile = &red_tiles.first().unwrap();

    let filled_in = last_tile.tile_to_inclusive(first_tile);

    for tile in filled_in.iter() {
        set.insert(tile.clone());
    }

    set
}

#[derive(Debug)]
struct Bounds {
    min_x: Option<u32>,
    max_x: Option<u32>,
    min_y: Option<u32>,
    max_y: Option<u32>,
}

impl Bounds {
    fn new() -> Self {
        Self {
            min_x: None,
            max_x: None,
            min_y: None,
            max_y: None,
        }
    }

    fn update(&mut self, position: &Position) {
        self.min_x = match self.min_x {
            Some(current_min) => Some(current_min.min(position.x)),
            None => Some(position.x),
        };

        self.max_x = match self.max_x {
            Some(current_max) => Some(current_max.max(position.x)),
            None => Some(position.x),
        };

        self.min_y = match self.min_y {
            Some(current_min) => Some(current_min.min(position.y)),
            None => Some(position.y),
        };

        self.max_y = match self.max_y {
            Some(current_max) => Some(current_max.max(position.y)),
            None => Some(position.y),
        };
    }
}

fn check_point_is_inside(
    position: &Position,
    bounds: &Bounds,
    outer_set: &HashSet<Position>,
) -> bool {
    //early check if already on boundary then must be inside
    if outer_set.contains(position) {
        return true;
    }
    //ray casting
    let max_x = bounds.max_x.expect("Should be populated!") + 1;

    let mut crossed_boundary = 0;

    for x_coord in position.x..=max_x {
        let temp_position = Position {
            x: x_coord,
            y: position.y,
        };
        //check if we hit the boundary
        if outer_set.contains(&temp_position) {
            crossed_boundary += 1;
        }
    }
    crossed_boundary % 2 != 0
}

fn check_rectangle_is_inside(
    rectangle: &Rectangle,
    bounds: &Bounds,
    outer_set: &HashSet<Position>,
) -> bool {
    //ray casting
    let positions = rectangle.get_positions(false);
    //println!("Postions: {:?}", positions);
    for position in positions {
        let result = check_point_is_inside(&position, bounds, outer_set);
        if !result {
            return false;
        }
    }
    true
}

fn main() {
    let input_path = "./input.txt";
    let input = std::fs::read_to_string(input_path).expect("Failed to read input file");

    let mut tiles: Vec<Position> = Vec::new();
    let mut outer_bounds: Bounds = Bounds::new();
    for (id, line) in input.lines().enumerate() {
        let position = Position::from_str(line);
        outer_bounds.update(&position);
        tiles.push(position);
    }

    //println!("Outer Bounds: {:?}", outer_bounds);

    let outer_set = fill_in_tiles(&tiles);

    //println!("Outer Tiles {:?}", outer_set);

    let mut rectangles: Vec<Rectangle> = Vec::new();
    for i in 0..tiles.len() {
        for j in (i + 1)..tiles.len() {
            let pos1 = &tiles[i];
            let pos2 = &tiles[j];
            let area = pos1.find_area(&pos2);
            let dist = Rectangle {
                position1: pos1.clone(),
                position2: pos2.clone(),
                area: area,
            };
            rectangles.push(dist);
        }
    }

    // //println!("rectangles {:?}", &rectangles);

    // Sot by area
    rectangles.sort_by_key(|rect| rect.area);
    rectangles.reverse();

    for rectangle in rectangles.iter() {
        if check_rectangle_is_inside(rectangle, &outer_bounds, &outer_set) {
            println!("Found valid rectangle: {:?}", rectangle);
            break;
        } else {
            println!("Rejected: {:?}", rectangle);
        }
    }

    // //list top
    // println!("top rectangles: {:?}", rectangles);
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_tile_to_inclusive() {
        let position1 = Position { x: 1, y: 5 };
        let position2 = Position { x: 1, y: 7 };
        let output = position1.tile_to_inclusive(&position2);
        let expected = vec![
            Position { x: 1, y: 5 },
            Position { x: 1, y: 6 },
            Position { x: 1, y: 7 },
        ];
        assert_eq!(output, expected);

        let position1 = Position { x: 1, y: 7 };
        let position2 = Position { x: 1, y: 5 };
        let output = position1.tile_to_inclusive(&position2);
        let expected = vec![
            Position { x: 1, y: 7 },
            Position { x: 1, y: 6 },
            Position { x: 1, y: 5 },
        ];
        assert_eq!(output, expected);
    }
}
