struct IdRange {
    start: u128,
    end: u128,
    invalid_ids: Vec<u128>,
}

impl IdRange {
    fn new(start: u128, end: u128) -> Self {
        IdRange {
            start,
            end,
            invalid_ids: Vec::new(),
        }
    }

    fn from_str(range_str: &str) -> Self {
        let parts: Vec<&str> = range_str.split('-').collect();
        let start = parts[0].parse::<u128>();

        let range_start = match start {
            Err(range_start) => panic!("Invalid start range: {}", range_start),
            Ok(range_start) => range_start,
        };

        let end = parts[1].parse::<u128>();

        let range_end = match end {
            Err(range_end) => panic!("Invalid end range: {}", range_end),
            Ok(range_end) => range_end,
        };


        IdRange::new(range_start, range_end)
    }

    fn is_invalid(&self, id: u128) -> bool {
        let string_id = id.to_string();
        // check length of id
        let string_len = string_id.len();

        if string_len % 2 == 0 {
            return has_repeated_digits(string_id);
        } else {
            return false;
        }
    }

    fn find_invalid_ids(&mut self) {
        for id in self.start..=self.end {
            if self.is_invalid(id) {
                self.invalid_ids.push(id);
            }
        }
    }
}

fn has_repeated_digits(id: String) -> bool {
    // split string in half
    let len = id.len();
    let half_len = len / 2;
    let first_half = &id[0..half_len];
    let second_half = &id[half_len..len];

    //check for equality
    first_half == second_half
}

fn main() {
    let input_path = "./input.txt";
    let input = std::fs::read_to_string(input_path).expect("Failed to read input file");

    // split input on commas
    let parts: Vec<&str> = input.trim().split(',').collect();
    let mut ranges = Vec::<IdRange>::new();
    let mut invalid_ids: Vec<u128> = Vec::new();

    for part in parts {
        println!("Processing range: {}", part);
        let mut range = IdRange::from_str(part);
        range.find_invalid_ids();
        invalid_ids.extend(range.invalid_ids.clone());

        ranges.push(range);
    }

    //println!("Invalid IDs found: {:?}", invalid_ids);

    let invalid_id_sum = invalid_ids.iter().sum::<u128>();
    println!("Sum of invalid IDs: {}", invalid_id_sum);
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_has_repeated_digits() {
        assert_eq!(has_repeated_digits("1212".to_string()), true);
        assert_eq!(has_repeated_digits("1234".to_string()), false);
        assert_eq!(has_repeated_digits("1111".to_string()), true);
        assert_eq!(has_repeated_digits("123123".to_string()), true);
    }

    #[test]
    fn test_is_invalid() {
        let id_range = IdRange::new(1000, 2000);
        assert_eq!(id_range.is_invalid(1212), true);
        assert_eq!(id_range.is_invalid(1234), false);
        assert_eq!(id_range.is_invalid(1111), true);
        assert_eq!(id_range.is_invalid(12345), false);
    }

    #[test]
    fn test_from_str() {
        let id_range = IdRange::from_str("100-200");
        assert_eq!(id_range.start, 100);
        assert_eq!(id_range.end, 200);
    }

    #[test]
    fn test_find_invalid_ids() {
        let mut id_range = IdRange::new(95, 115);
        id_range.find_invalid_ids();
        assert_eq!(id_range.invalid_ids, vec![99]);
    }
}
