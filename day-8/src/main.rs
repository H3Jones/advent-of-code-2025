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
}