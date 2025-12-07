use std::{collections::HashMap, vec};

#[derive(Debug, PartialEq, Eq, Clone)]
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

    fn calculate(&self) -> u128 {
        match self.operator {
            Operator::Add => self.values.iter().map(|&v| v as u128).sum(),
            Operator::Multiply => self.values.iter().map(|&v| v as u128).product(),
            Operator::NotSet => panic!("Operator not set for calculation"),
        }
    }
}

fn main() {
    let input_path = "./input_short.txt";
    let input = std::fs::read_to_string(input_path).expect("Failed to read input file");

    let mut values_vec: Vec<&str> = Vec::new();
    let mut operators_vec: Vec<&str> = Vec::new();
    let mut values_line_count = 0;
    let mut operators_line_count = 0;

    for line in input.lines() {
        println!("{line}");
        // if lines contains values
        if !line.contains('+') && !line.contains('*') {
            values_vec.push(line);
            values_line_count += 1;
        } else {
            operators_vec.push(line);
            operators_line_count += 1;
        }
    }

    println!(
        "Found {} values lines and {} operators lines",
        values_line_count, operators_line_count
    );

    let col_indices = get_col_indices(operators_vec[0]);
    println!("Indices: {:?}", col_indices);
}

#[derive(Debug, PartialEq, Eq)]
struct ColIndex {
    start: usize,
    end: usize,
}

fn get_col_indices(line: &str) -> Vec<ColIndex> {
    let mut col_indices: Vec<usize> = Vec::new();
    for (idx, character) in line.char_indices() {
        if character == '+' || character == '*' {
            col_indices.push(idx);
        }
    }
    let mut output: Vec<ColIndex> = Vec::new();
    let mut start_idx = col_indices[0];
    for x in col_indices.iter().skip(1) {
        let end_idx = x - 1;
        output.push(ColIndex {
            start: start_idx,
            end: end_idx,
        });
        start_idx = *x;
    }
    output
}

fn process_colwise(input: Vec<String>, operators_vec: String, col_indices: Vec<ColIndex>) -> Vec<Vec<u32>> {
    let mut output: Vec<Vec<u32>> = Vec::new();

    for idx in col_indices.iter() {
        let col_start = idx.start;
        let col_end = idx.end;
        for col in col_end..col_start {
            
        }
    }

    output
}

fn get_column(input: &Vec<String>, col: usize) -> Vec<char> {
    let mut output: Vec<char> = Vec::new();
    for line in input {
        if let Some(ch) = line.chars().nth(col) {
            output.push(ch);
        }
    }
    output
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

fn lengths<T>(vec: &Vec<Vec<T>>) -> Vec<usize> {
    vec.iter().map(|v| v.len()).collect()
}

#[cfg(test)]
mod tests {
    use std::convert;

    use super::*;
    #[test]
    fn test_calculation_add() {
        let calc = Calculation::new(vec![1, 2, 3, 4], Operator::Add);
        let result = calc.calculate();
        assert_eq!(result, 10);
        let calc_mul = Calculation::new(vec![1, 2, 3, 4], Operator::Multiply);
        let result_mul = calc_mul.calculate();
        assert_eq!(result_mul, 24);
    }
    #[test]
    fn test_get_col_indices() {
        let line = "*   +   *   +  ";
        let indices = get_col_indices(line);
        let expected = vec![
            ColIndex { start: 0, end: 3 },
            ColIndex { start: 4, end: 7 },
            ColIndex { start: 8, end: 11 },
        ];
        assert_eq!(indices, expected);
    }
    #[test]
    fn test_get_column() {
        let lines = vec!["123".to_string(), " 45".to_string(), "  6".to_string()];
        assert_eq!(get_column(&lines, 0), vec!['1', ' ', ' ']);
        assert_eq!(get_column(&lines, 1), vec!['2', '4', ' ']);
        assert_eq!(get_column(&lines, 2), vec!['3', '5', '6']);
    }
}
