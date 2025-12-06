use std::collections::HashMap;

#[derive(Debug, PartialEq, Eq)]
enum Operator {
    Add,
    Multiply,
    NotSet,
}

impl Operator {
    fn from_char(op_char: char) -> Operator {
        match op_char {
            '+' => Operator::Add,
            '*' => Operator::Multiply,
            _ => panic!("Invalid operator character: {}", op_char),
        }
    }
}

struct Calculation {
    values: Vec<u32>,
    operator: Operator,
}

impl Calculation {
    fn new(values: Vec<u32>, operator: Operator) -> Self {
        Calculation { values, operator }
    }
}

fn main() {
    let input_path = "./input_short.txt";
    let input = std::fs::read_to_string(input_path).expect("Failed to read input file");

    let mut values_vec: Vec<Vec<u32>> = Vec::new();
    let mut operators_vec: Vec<Vec<Operator>> = Vec::new();
    let mut values_line_count = 0;
    let mut operators_line_count = 0;

    for line in input.lines() {
        // if lines contains values
        if !line.contains('+') && !line.contains('*') {
            let values = process_values_line(line);
            println!("Processed values line: {:?}", values);
            values_vec.push(values);
            values_line_count += 1;
        } else {
            let operators = process_operator_line(line);
            println!("Processed operators line: {:?}", &operators);
            operators_vec.push(operators);
            operators_line_count += 1;
        }
    }
    println!(
        "Found {} values lines and {} operators lines",
        values_line_count, operators_line_count
    );
}

fn process_values_line(line: &str) -> Vec<u32> {
    line.split_whitespace()
        .map(|s| {
            s.trim()
                .parse::<u32>()
                .expect("Invalid number in values line")
        })
        .collect()
}

fn process_operator_line(line: &str) -> Vec<Operator> {
    line.split_whitespace()
        .map(|s| {
            let ch = s.trim().chars().next().expect("Empty operator string");
            Operator::from_char(ch)
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_process_values_line() {
        let line = "10 20 30 40";
        let values = process_values_line(line);
        assert_eq!(values, vec![10, 20, 30, 40]);
    }
    #[test]
    fn test_process_operator_line() {
        let line = "+ * +";
        let operators = process_operator_line(line);
        assert_eq!(operators.len(), 3);
        let expected = vec![Operator::Add, Operator::Multiply, Operator::Add];
        assert_eq!(operators, expected);
    }
}
