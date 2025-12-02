struct IdRange {
    start: u32,
    end: u32,
    invalid_ids: Vec<u32>,
}

impl IdRange {
    fn new(start: u32, end: u32) -> Self {
        IdRange {
            start,
            end,
            invalid_ids: Vec::new(),
        }
    }

    fn is_invalid(&self, id: u32) -> bool {
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
    println!("Hello, world!");
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
}
