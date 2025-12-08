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
        let x_comp = (other.x - self.x).pow(2);
        let y_comp = (other.y - self.y).pow(2);
        let z_comp = (other.z - self.z).pow(2);
        let sum = (x_comp + y_comp + z_comp) as f32;
        sum.sqrt()
    }
}

fn main() {
    let input_path = "./input_short.txt";
    let input = std::fs::read_to_string(input_path).expect("Failed to read input file");

    let mut positions: Vec<Position> = Vec::new();
    for line in input.lines() {
        positions.push(Position::from_str(line));
    }

    println!("Positions {:?}", positions);
}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_position_from_str() {
        let pos = Position::from_str("123,456,789");
        let expected = Position { x: 123, y: 456, z: 789 };
        assert_eq!(pos, expected);
    }
    #[test]
    fn test_distance_to() {
        let position1 = Position {x: 0, y: 0, z: 0};
        let position2 = Position {x: 4, y: 4, z: 4};
        let distance = position1.distance_to(&position2);
        assert_eq!(distance.round(), 7.0);
    }
}