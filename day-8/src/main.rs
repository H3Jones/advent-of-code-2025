use std::collections::HashMap;

#[derive(Debug, PartialEq, Eq)]
struct Position {
    x: u32,
    y: u32,
    z: u32,
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
            z: values[2],
        }
    }

    fn distance_to(&self, other: &Position) -> f32 {
        let dx = self.x.abs_diff(other.x) as f64;
        let dy = self.y.abs_diff(other.y) as f64;
        let dz = self.z.abs_diff(other.z) as f64;

        (dx * dx + dy * dy + dz * dz).sqrt() as f32
    }
}

#[derive(Debug)]
struct JunctionBox {
    id: u32,
    position: Position,
}

#[derive(Debug)]
struct Distance {
    ids: (u32, u32),
    value: f32,
}

fn main() {
    let input_path = "./input_short.txt";
    let input = std::fs::read_to_string(input_path).expect("Failed to read input file");

    let mut boxes: Vec<JunctionBox> = Vec::new();
    for (id, line) in input.lines().enumerate() {
        let position = Position::from_str(line);
        boxes.push(JunctionBox {
            id: id as u32,
            position: position,
        });
    }

    println!("Boxes {:?}", boxes);

    let mut distances: HashMap<(u32, u32), f32> = HashMap::new();
    for i in 0..boxes.len() {
        for j in (i + 1)..boxes.len() {
            let pos1 = &boxes[i];
            let pos2 = &boxes[j];
            let distance = pos1.position.distance_to(&pos2.position);
            distances.insert((i as u32, j as u32), distance);
        }
    }

    println!("distances: {:?}", distances);
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_position_from_str() {
        let pos = Position::from_str("123,456,789");
        let expected = Position {
            x: 123,
            y: 456,
            z: 789,
        };
        assert_eq!(pos, expected);
    }
    #[test]
    fn test_distance_to() {
        let position1 = Position { x: 0, y: 0, z: 0 };
        let position2 = Position { x: 4, y: 4, z: 4 };
        let distance = position1.distance_to(&position2);
        assert_eq!(distance.round(), 7.0);
    }
}
