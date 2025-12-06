use std::collections::HashMap;

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
    let input_path = "./input.txt";
    let input = std::fs::read_to_string(input_path).expect("Failed to read input file");

    let mut values_vec: Vec<Vec<u32>> = Vec::new();
    let mut operators_vec: Vec<Vec<Operator>> = Vec::new();
    let mut values_line_count = 0;
    let mut operators_line_count = 0;

    for line in input.lines() {
        // if lines contains values
        if !line.contains('+') && !line.contains('*') {
            let values = process_values_line(line);
            //println!("Processed values line: {:?}", values);
            values_vec.push(values);
            values_line_count += 1;
        } else {
            let operators = process_operator_line(line);
            //println!("Processed operators line: {:?}", &operators);
            operators_vec.push(operators);
            operators_line_count += 1;
        }
    }
    println!(
        "Found {} values lines and {} operators lines",
        values_line_count, operators_line_count
    );

    //check all lines have same length
    let len_operators: Vec<usize> = lengths(&operators_vec);
    let len_values: Vec<usize> = lengths(&values_vec);

    println!("Values line lengths: {:?}", len_values);
    println!("Operators line count: {:?}", len_operators);

    let operators = &operators_vec[0];

    // Create calculations vec
    let mut calculations: Vec<Calculation> = Vec::new();
    let mut answers: Vec<u128> = Vec::new();
    for (i, operator) in operators.iter().enumerate() {
        let mut values_for_calc: Vec<u32> = Vec::new();
        for values_line in &values_vec {
            if i < values_line.len() {
                values_for_calc.push(values_line[i]);
            } else {
                panic!("Mismatch in values line length for operator index {}", i);
            }
        }
        let calc = Calculation::new(values_for_calc, operator.clone());
        answers.push(calc.calculate());
        calculations.push(calc);
    }

    //sum answers
    let total_answer: u128 = answers.iter().sum();
    println!("Total answer from calculations: {}", total_answer);

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
    #[test]
    fn test_calculation_add() {
        let calc = Calculation::new(vec![1, 2, 3, 4], Operator::Add);
        let result = calc.calculate();
        assert_eq!(result, 10);
        let calc_mul = Calculation::new(vec![1, 2, 3, 4], Operator::Multiply);
        let result_mul = calc_mul.calculate();
        assert_eq!(result_mul, 24);
    }
}
