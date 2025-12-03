struct BatteryBank {
    batteries: Vec<u32>,
    max_joltage: u32,
}

impl BatteryBank {
    fn new(batteries: Vec<u32>) -> Self {
        BatteryBank {
            batteries,
            max_joltage: 0,
        }
    }

    fn from_str(battery_str: &str) -> Self {
        let batteries: Vec<u32> = battery_str
            .chars()
            .map(|c| c.to_digit(10).expect("Invalid digit"))
            .collect();
        BatteryBank {
            batteries,
            max_joltage: 0,
        }
    }

    fn calculate_max_joltage(&mut self) {
        self.max_joltage = find_max_digits(&self.batteries);
    }
}

fn find_max_digits(digits: &Vec<u32>) -> u32 {
    let mut first_max = 0;
    let mut first_position = 0;

    let mut second_max = 0;
    let mut _second_position = 0;

    let vec_len = digits.len();

    //iterate front to back
    for i in 0..digits.len() - 1 {
        let digit = digits[i];

        // update first and second max accordingly
        if digit > first_max {
            first_max = digit;
            first_position = i;
        }
        // if digit is 9 we can exit early
        if first_max == 9 {
            break;
        }
    }

    //iterate front to back from the position after first max
    for i in first_position + 1..vec_len {
        let digit = digits[i];
        if digit > second_max {
            second_max = digit;
            _second_position = i;
        }
        // if second max is 9 we can exit early
        if second_max == 9 {
            break;
        }
    }

    assert!(
        first_position < _second_position,
        "First max position should be less than second max position"
    );

    first_max * 10 + second_max
}

fn main() {
    let input_path = "./input.txt";
    let input = std::fs::read_to_string(input_path).expect("Failed to read input file");

    let mut banks: Vec<BatteryBank> = input
        .lines()
        .map(|line| BatteryBank::from_str(line))
        .collect();

    println!("Parsed {} battery banks", banks.len());

    for (i, bank) in banks.iter_mut().enumerate() {
        bank.calculate_max_joltage();
        println!("Battery Bank {}: Max Joltage = {}", i + 1, bank.max_joltage);
    }

    //total sum of max joltages
    let total_max_joltage: u32 = banks.iter().map(|bank| bank.max_joltage).sum();
    println!("Total Max Joltage: {}", total_max_joltage);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_battery_bank_from_str() {
        let battery_str = "1234567890";
        let bank = BatteryBank::from_str(battery_str);
        assert_eq!(bank.batteries, vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 0]);
    }

    #[test]
    fn test_battery_bank_new() {
        let batteries = vec![5, 10, 15];
        let bank = BatteryBank::new(batteries.clone());
        assert_eq!(bank.batteries, batteries);
    }

    #[test]
    fn test_find_max_digits() {
        let digits = vec![1, 3, 5, 7, 9];
        let result = find_max_digits(&digits);
        assert_eq!(result, 79);

        let digits2 = vec![9, 8, 7, 6, 5];
        let result2 = find_max_digits(&digits2);
        assert_eq!(result2, 98);

        let digits3 = vec![8, 1, 8, 1, 8, 1, 9, 1, 1, 1, 1, 2, 1, 1, 1];
        let result3 = find_max_digits(&digits3);
        assert_eq!(result3, 92);
    }
}
