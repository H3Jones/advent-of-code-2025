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
}

#[derive(Debug)]
struct Rectangle {
    position1: Position,
    position2: Position,
    area: i64,
}

fn main() {
    let input_path = "./input.txt";
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
