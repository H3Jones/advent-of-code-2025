use std::collections::{HashMap, HashSet};

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

    // Leave unrooted so we dont worry about floats
    fn distance_to(&self, other: &Position) -> i32 {
        let dx = self.x.abs_diff(other.x) as i32;
        let dy = self.y.abs_diff(other.y) as i32;
        let dz = self.z.abs_diff(other.z) as i32;

        dx * dx + dy * dy + dz * dz
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
    value: i32,
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

    //println!("Boxes {:?}", boxes);

    let mut distances: Vec<Distance> = Vec::new();
    for i in 0..boxes.len() {
        for j in (i + 1)..boxes.len() {
            let pos1 = &boxes[i];
            let pos2 = &boxes[j];
            let distance = pos1.position.distance_to(&pos2.position);
            let dist = Distance {
                ids: (i as u32 + 1, j as u32 + 1),
                value: distance,
            };
            distances.push(dist);
        }
    }

    // Sot by distance
    distances.sort_by_key(|dist| dist.value);

    //Get top 10
    distances.truncate(11);

    println!("top ten distances: {:?}", distances);

    let mut results: Vec<HashSet<u32>>= Vec::new();
    let mut visited: HashSet<IdPair> = HashSet::new();

    for dist in distances.iter() {

        let ids = dist.ids;

        if visited.contains(&ids) {
            continue;
        }

        let mut set: HashSet<u32> = HashSet::new();
        //insert ids
       
        set.insert(ids.0);
        set.insert(ids.1);
        for second_dist in distances.iter() {
            let second_ids = second_dist.ids;
            if set.contains(&second_ids.0) || set.contains(&second_ids.1) {
                set.insert(second_ids.0);
                set.insert(second_ids.1);
                visited.insert(second_ids);
            }
            
        }
        println!("Current set: {:?}",set);
        results.push(set);
    };
    println!("{:?}", results);

    //answer is product of lens
    let mut totals: Vec<u32> = results.iter().map(|set| set.len() as u32).collect();
    totals.sort();
    totals.reverse();
    println!("totals: {:?}", totals);
    totals.truncate(3);
    let total: u32 = totals.iter().product();
    println!("Total: {total}");
}

type IdPair = (u32, u32);

fn id_match(a: IdPair, b: IdPair) -> bool {
    // Check if the first element either element of b
    (a.0 == b.0) || (a.0 == b.1) ||
    // Check if the second element matches either element of b
    (a.1 == b.0) || (a.1 == b.1)
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
        assert_eq!(distance, 48);
    }
    #[test]
    fn test_id_match() {
        assert!(id_match((0, 5), (1, 5)));
        assert_ne!((0, 5), (10, 11));
    }
}
