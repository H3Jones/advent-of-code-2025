#[derive(Clone, Debug, PartialEq, Eq)]
struct IdRange {
    start: u64,
    end: u64,
}
impl IdRange {
    fn new(start: u64, end: u64) -> Self {
        assert!(start <= end, "Start of range must be less than or equal to end: {} - {}", start, end);
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

    fn number_ids_in_range(&self) -> u64 {
        self.end - self.start + 1
    }
}

fn merge_ranges(ranges: &mut Vec<IdRange>) -> Vec<IdRange> {
    if ranges.is_empty() {
        return Vec::new();
    }

    // Sort ranges by start value
    ranges.sort_by_key(|r| r.start);

    let mut merged_ranges: Vec<IdRange> = Vec::new();
    let mut current_range = ranges[0].clone();

    for range in ranges.iter().skip(1) {
        if range.start <= current_range.end + 1 {
            // Ranges overlap so merge them
            current_range.end = current_range.end.max(range.end);
        } else {
            // No overlap, add the current range to the list and start a new one
            merged_ranges.push(current_range);
            current_range = range.clone();
        }
    }

    // Add the last range
    merged_ranges.push(current_range);

    merged_ranges
}

fn main() {
    let input_path = "./input.txt";
    let input = std::fs::read_to_string(input_path).expect("Failed to read input file");
   
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

    // merge overlapping ranges
    ranges = merge_ranges(&mut ranges);

    println!("Merged to {} ranges", ranges.len());

    println!("Loaded {} IDs", ids.len());

    let mut total_ids_in_ranges: u128 = 0;

    let mut invalid_ids = ids.clone();
    for range in ranges.iter() {        
        // remove any ids that are in range
        invalid_ids.retain(|&id| !range.is_in_range(id));
        println!("Filtering IDs for range {}-{}: items remaining: {}", range.start, range.end, invalid_ids.len());

        total_ids_in_ranges += range.number_ids_in_range() as u128;
    }

    println!("Found {} invalid IDs:", invalid_ids.len());

    //println!("{:?}", invalid_ids);

    println!("Found {} valid IDs:", ids.len() - invalid_ids.len());

    println!("Total IDs in ranges: {}", total_ids_in_ranges);
    
    
}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_is_in_range() {
        let id_range = IdRange::new(100, 200);
        assert_eq!(id_range.is_in_range(150), true);
        assert_eq!(id_range.is_in_range(99), false);
        assert_eq!(id_range.is_in_range(201), false);
    }
    #[test]
    fn test_is_in_single_value_range() {
        let id_range = IdRange::new(100, 100);
        assert_eq!(id_range.is_in_range(100), true);
        assert_eq!(id_range.is_in_range(99), false);
        assert_eq!(id_range.is_in_range(101), false);
    }
    #[test]
    fn test_number_ids_in_range() {
        let id_range = IdRange::new(100, 200);
        assert_eq!(id_range.number_ids_in_range(), 101);
        let single_value_range = IdRange::new(100, 100);
        assert_eq!(single_value_range.number_ids_in_range(), 1);
    }
    #[test]
    fn test_merge_ranges() {
        let mut ranges = vec![
            IdRange::new(100, 200),
            IdRange::new(150, 250),
            IdRange::new(300, 400),
            IdRange::new(350, 450),
        ];
        let merged = merge_ranges(&mut ranges);
        assert_eq!(merged.len(), 2);
        let expected = vec![
            IdRange::new(100, 250),
            IdRange::new(300, 450),
        ];
        assert_eq!(merged, expected);
    }
}