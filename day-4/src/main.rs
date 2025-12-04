use std::{char, collections::VecDeque};

fn main() {
    let input_path = "./input_short.txt";
    let input = std::fs::read_to_string(input_path).expect("Failed to read input file");

    let output: Vec<Vec<char>> = input
        .lines()
        .map(|line| {
            let chars: Vec<char> = line.chars().collect();
            //println!("Processing line {}: {:?}", line_number, chars);
            chars
        })
        .collect();

    //print first ten lines of output
    for i in 0..10.min(output.len()) {
        println!("Line {}: {:?}", i + 1, output[i]);
    }

    //three buffers
    let mut line_above_buffer: Vec<char> = Vec::new(); //minus one
    let mut current_line_buffer: Vec<char> = Vec::new(); //row of interest
    let mut next_line_buffer: Vec<char> = Vec::new(); //plus one

    for row in 0..output.len() {
        //first row special case
        line_above_buffer = if row > 0 {
            current_line_buffer.clone()
        } else {
            Vec::new()
        };
        current_line_buffer = output[row].clone();
        //last row special case
        next_line_buffer = if row + 1 < output.len() {
            output[row + 1].clone()
        } else {
            Vec::new()
        };

        // println!(
        //     "Above: {:?}\nCurrent: {:?}\nBelow: {:?}\n",
        //     line_above_buffer, current_line_buffer, next_line_buffer
        // );
        let processed = process_line(&line_above_buffer, &current_line_buffer, &next_line_buffer);
        println!("Processed Line {}: {:?}", row + 1, processed);
    }
}

fn process_line(above: &Vec<char>, current: &Vec<char>, below: &Vec<char>) -> Vec<char> {
    let max_len = current.len(); //assuming constant line length
    let mut result_line: Vec<char> = current.clone();

    let max_surrounding = 3; //max number of surrounding cells containing paper

    let mut window = Window::new();
    window.init(&'.');    

    for col in 0..max_len {
        // if first col special case we need to init the window
        if col == 0 {
            let column = get_column(above, current, below, col);
            window.add_column(column);
        }
        
        //push in the next column

        result_line[col] = 'X'; //placeholder
    }
    result_line
}

struct Column {
    top: char,
    middle: char,
    bottom: char,
}

fn get_column(
    above: &Vec<char>,
    current: &Vec<char>,
    below: &Vec<char>,
    col: usize,
) -> Column {
    let top = if !above.is_empty() { above[col] } else { '.' };
    let middle = current[col];
    let bottom = if !below.is_empty() { below[col] } else { '.' };
    Column { top, middle, bottom }
}

struct Window {
    deque: VecDeque<Column>,
}

impl Window {
    fn new() -> Self {
        Self {
            deque: VecDeque::with_capacity(3)
        }
    }

    fn init(&mut self, char: &char) {
        for _ in 0..3 {
            self.deque.push_back(Column {
                top: *char,
                middle: *char,
                bottom: *char,
            });
        }
    }

    fn add_column(&mut self, column: Column) {
        if self.deque.len() == 3 {
            self.deque.pop_front();
        }
        self.deque.push_back(column);
    }

    fn count_occupied(&self) -> usize {
        let mut count = 0;
        // count across all three columns in the window
        for (idx, col) in self.deque.iter().enumerate() {
            if col.top != '.' {
                count += 1;
            }
            if idx != 1 && col.middle != '.' {
                count += 1;
            }
            if col.bottom != '.' {
                count += 1;
            }
        }
        count
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_window_init() {
        let mut window = Window::new();
        window.init(&'.');
        assert_eq!(window.deque.len(), 3);
        for col in window.deque.iter() {
            assert_eq!(col.top, '.');
            assert_eq!(col.middle, '.');
            assert_eq!(col.bottom, '.');
        }
    }
    #[test]
    fn test_window_count() {
        let mut window = Window::new();
        window.add_column(Column { top: 'A', middle: 'B', bottom: '.' });
        window.add_column(Column { top: 'B', middle: 'C', bottom: 'D' });
        window.add_column(Column { top: '.', middle: '.', bottom: 'E' });
        assert_eq!(window.count_occupied(), 5);
    }
}
