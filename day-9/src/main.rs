#[derive(Debug, PartialEq, Clone)]
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

fn fill_in_tiles(red_tiles: &Vec<Position>) {
    for (idx, tile) in red_tiles.iter().enumerate().skip(1) {
        let tile1 = &red_tiles[idx - 1];
        let tile2 = &red_tiles[idx];

        //check if tiles in are same row or same column
    }
}

fn main() {
    let input_path = "./input_short.txt";
    let input = std::fs::read_to_string(input_path).expect("Failed to read input file");

    let mut tiles: Vec<Position> = Vec::new();
    for (id, line) in input.lines().enumerate() {
        let position = Position::from_str(line);
        tiles.push(position);
    }

    //println!("Tiles {:?}", tiles);

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

    //println!("rectangles {:?}", &rectangles);

    // Sot by area
    rectangles.sort_by_key(|rect| rect.area);
    rectangles.reverse();
    rectangles.truncate(1);

    //list top
    println!("top rectangles: {:?}", rectangles);
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
