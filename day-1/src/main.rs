#[derive(Debug)]
enum Direction {
    Left,
    Right,
}

#[derive(Debug)]
struct Rotation {
    // Direction to rotate
    direction: Direction,
    // Number of clicks to rotate
    clicks: u32,
}

#[derive(Debug)]
struct Lock {
    // Current position of the lock (0-99)
    dial_position: u32,
    // zero-counter (how many times we've passed 0)
    zero_passed_counter: u32,
    // Number of times the dial has landed on 0
    zero_finished_counter: u32,
}

impl Lock {
    fn new() -> Self {
        Lock {
            dial_position: 50,
            zero_passed_counter: 0,
            zero_finished_counter: 0,
        }
    }

    // Rotate the lock one click to the right
    fn rotate_one_right(&mut self) {
        if self.dial_position == 99 {
            self.dial_position = 0;
            // Increment zero_passed_counter when passing 0
            self.zero_passed_counter += 1;
        } else {
            self.dial_position += 1;
        }
    }
    // Rotate the lock one click to the left
    fn rotate_one_left(&mut self) {
        if self.dial_position == 0 {
            self.dial_position = 99;
        } else if self.dial_position == 1 {
            self.dial_position = 0;
            // Increment zero_passed_counter when passing 0
            self.zero_passed_counter += 1;
        } else {
            self.dial_position -= 1;
        }
    }

    fn rotate(&mut self, rotation: &Rotation) {
        match rotation.direction {
            Direction::Left => {
                for _ in 0..rotation.clicks {
                    self.rotate_one_left();
                }
            }
            Direction::Right => {
                for _ in 0..rotation.clicks {
                    self.rotate_one_right();
                }
            }
        }
        println!(
            "Rotated {:?} by {} clicks to position {}",
            rotation.direction, rotation.clicks, self.dial_position
        );

        if self.dial_position == 0 {
            self.zero_finished_counter += 1;
            println!(
                "Rotation ended on 0, incrementing counter to {}",
                self.zero_finished_counter
            );
        }
    }
}

fn parse_rotations(input: &str) -> Vec<Rotation> {
    input
        .lines()
        .map(|line| {
            println!("Parsing line: {}", line);
            let direction_str = &line[0..1];
            let clicks_str = &line[1..];

            let direction = match direction_str {
                "L" => Direction::Left,
                "R" => Direction::Right,
                _ => panic!("Invalid direction"),
            };

            let clicks: u32 = clicks_str.parse().expect("Invalid number of clicks");

            Rotation { direction, clicks }
        })
        .collect()
}

fn main() {
    let input_path = "./input.txt";
    let input = std::fs::read_to_string(input_path).expect("Failed to read input file");

    let rotations = parse_rotations(&input);

    let lock = &mut Lock::new();

    for rotation in &rotations {
        lock.rotate(rotation);
    }

    println!(
        "Final lock state: position {}, zero_passed_counter {}, zero_finished_counter {}, total zero events {}",
        lock.dial_position,
        lock.zero_passed_counter,
        lock.zero_finished_counter,
        lock.zero_passed_counter + lock.zero_finished_counter
    );
}
