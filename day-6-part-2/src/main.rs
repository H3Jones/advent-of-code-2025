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

#[derive(Debug)]
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

    let mut values_vec: Vec<String> = Vec::new();
    let mut operators_vec: Vec<String> = Vec::new();
    let mut values_line_count = 0;
    let mut operators_line_count = 0;

    for line in input.lines() {
        //println!("{line}");
        // if lines contains values
        if !line.contains('+') && !line.contains('*') {
            values_vec.push(line.to_string());
            values_line_count += 1;
        } else {
            operators_vec.push(line.to_string());
            operators_line_count += 1;
        }
    }

    println!(
        "Found {} values lines and {} operators lines",
        values_line_count, operators_line_count
    );

    let col_indices = get_col_indices(&operators_vec[0]);
    //println!("Indices: {:?}", col_indices);

    let processed = process_colwise(
        values_vec,
        operators_vec[0].clone(),
        col_indices,
    );

    //println!("processed {:?}", processed);

    let mut answers: Vec<u128> = Vec::new();
    for calc in processed.iter() {
        answers.push(calc.calculate());
    }

    //sum answers
    let total_answer: u128 = answers.iter().sum();
    println!("total: {total_answer}");
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
    let default_end = line.len();

    for (idx, &idx_value) in col_indices.iter().enumerate() {
        let col_start = idx_value;
        let col_end = col_indices.get(idx + 1).unwrap_or(&default_end) - 1;
        let col_index = ColIndex {
            start: col_start,
            end: col_end,
        };
        output.push(col_index);
    }

    output
}

fn process_colwise(
    input: Vec<String>,
    operators_vec: String,
    col_indices: Vec<ColIndex>,
) -> Vec<Calculation> {
    let mut output: Vec<Calculation> = Vec::new();
    let operators = process_operator_line(&operators_vec);

    for (idx_num, idx) in col_indices.iter().enumerate() {
        let col_start = idx.start;
        let col_end = idx.end;
        let mut values: Vec<u32> = Vec::new();
        for col in col_start..=col_end {
            let digits = get_column(&input, col);
            if let Some(num) = collapse_to_number(digits) {
                values.push(num);
            }
        }

        let calculation = Calculation {
            values,
            operator: operators[idx_num].clone(),
        };
        output.push(calculation);
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

fn collapse_to_number(digits: Vec<char>) -> Option<u32> {
    let digit_string: String = digits.into_iter().filter(|c| !c.is_whitespace()).collect();
    if digit_string.is_empty() {
        return None;
    }
    digit_string.parse::<u32>().ok()
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
            ColIndex { start: 12, end: 14 },
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
    #[test]
    fn test_collapse_to_number() {
        assert_eq!(collapse_to_number(vec!['1', ' ', '3']), Some(13));
        assert_eq!(collapse_to_number(vec![' ', ' ', '3']), Some(3));
        assert!(collapse_to_number(vec![' ', ' ', ' ']).is_none());
    }
}
