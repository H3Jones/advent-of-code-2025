struct IdRange {
    start: u64,
    end: u64,
}
impl IdRange {
    fn new(start: u64, end: u64) -> Self {
        IdRange { start, end }
    }

    fn is_in_range(&self, id: u64) -> bool {
        id >= self.start && id <= self.end
    }

    fn from_str(range_str: &str) -> Self {
        let parts: Vec<&str> = range_str.split('-').collect();
        let start = parts[0].parse::<u64>();

        let range_start = match start {
            Err(range_start) => panic!("Invalid start range: {}", range_start),
            Ok(range_start) => range_start,
        };

        let end = parts[1].parse::<u64>();

        let range_end = match end {
            Err(range_end) => panic!("Invalid end range: {}", range_end),
            Ok(range_end) => range_end,
        };

        IdRange::new(range_start, range_end)
    }
}

fn main() {
    let input_path = "./input.txt";
    let input = std::fs::read_to_string(input_path).expect("Failed to read input file");

    let mut valid_ids: Vec<u64> = Vec::new();
    let mut invalid_ids: Vec<u64> = Vec::new();

    let mut ranges: Vec<IdRange> = Vec::new();
    let mut ids: Vec<u64> = Vec::new();

    for line in input.lines() {
        match line {
            val if val.contains('-') => {
                let range = IdRange::from_str(line);
                ranges.push(range);
            }
            empty_val if empty_val.is_empty() => continue,
            _ => {
                let id = line.parse::<u64>();
                match id {
                    Err(e) => panic!("Invalid ID: {}", e),
                    Ok(valid_id) => ids.push(valid_id),
                }
            }
        }
    }

    println!("Loaded {} ranges", ranges.len());
    println!("Loaded {} IDs", ids.len());
}
