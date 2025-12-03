struct BatteryBank {
    batteries: Vec<u32>,
}

impl BatteryBank {
    fn new(batteries: Vec<u32>) -> Self {
        BatteryBank { batteries }
    }

    fn from_str(battery_str: &str) -> Self {
        let batteries: Vec<u32> = battery_str.chars().map(|c| c.to_digit(10).expect("Invalid digit")).collect();
        BatteryBank { batteries }
    }
}

fn main() {
    let input_path = "./input.txt";
    let input = std::fs::read_to_string(input_path).expect("Failed to read input file");

    let banks: Vec<BatteryBank> = input
        .lines()
        .map(|line| BatteryBank::from_str(line))
        .collect();

    println!("Parsed {} battery banks", banks.len());
    //print first 3 banks
    for (i, bank) in banks.iter().enumerate().take(3) {
        println!("Bank {}: {:?}", i + 1, bank.batteries);
    }
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
}