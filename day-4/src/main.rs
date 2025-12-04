use std::{char, collections::VecDeque};

fn main() {
    let input_path = "./input.txt";
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

    let mut value = perform_processing(output);

    let mut total_removed = 0;

    let mut iterations = 0;

    while value.items_removed > 0 {
        value = perform_processing(value.output).replace_xs_with_dots();
        iterations += 1;
        println!(
            "After iteration {}: {} items removed",
            iterations, value.items_removed
        );
        total_removed += value.items_removed;
    }
    println!(
        "Total items removed after {} iterations: {}",
        iterations, total_removed
    );
}

struct ProcessedOutput {
    output: Vec<Vec<char>>,
    items_removed: usize,
}

impl ProcessedOutput {
    fn replace_xs_with_dots(mut self) -> Self {
        for row in self.output.iter_mut() {
            for c in row.iter_mut() {
                if *c == 'X' {
                    *c = '.';
                }
            }
        }
        self
    }
}

fn perform_processing(input: Vec<Vec<char>>) -> ProcessedOutput {
    //three buffers
    let mut line_above_buffer: Vec<char>;
    //minus one
    let mut current_line_buffer: Vec<char> = Vec::new();
    //row of interest
    let mut next_line_buffer: Vec<char>;
    //plus one

    let mut output: Vec<Vec<char>> = Vec::new();
    let mut x_count = 0;

    for row in 0..input.len() {
        //first row special case
        line_above_buffer = if row > 0 {
            current_line_buffer.clone()
        } else {
            Vec::new()
        };
        current_line_buffer = input[row].clone();
        //last row special case
        next_line_buffer = if row + 1 < input.len() {
            input[row + 1].clone()
        } else {
            Vec::new()
        };

        // println!(
        //     "Above: {:?}\nCurrent: {:?}\nBelow: {:?}\n",
        //     line_above_buffer, current_line_buffer, next_line_buffer
        // );
        let processed = process_line(&line_above_buffer, &current_line_buffer, &next_line_buffer);

        //count X in processed line
        let line_x_count = processed.iter().filter(|&&c| c == 'X').count();

        println!(
            "Processed Line {}: {:?} found {} X",
            row + 1,
            processed,
            line_x_count
        );
        output.push(processed);

        x_count += line_x_count;
    }

    println!("Total X found: {}", x_count);

    ProcessedOutput {
        output: output,
        items_removed: x_count,
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

        //push in the next column to the right
        if col + 1 < max_len {
            let next_column = get_column(above, current, below, col + 1);
            window.add_column(next_column);
        } else {
            //push in empty column if we are at the edge
            window.add_column(Column {
                top: '.',
                middle: '.',
                bottom: '.',
            });
        }

        //println!("Window at col {}: {:?}", col, window.deque);

        if window.get_middle_value() == '.' {
            //current cell is empty, skip
            continue;
        }

        let occupied_count = window.count_occupied();

        if occupied_count <= max_surrounding {
            result_line[col] = 'X';
        }
    }
    result_line
}

#[derive(Debug)]
struct Column {
    top: char,
    middle: char,
    bottom: char,
}

fn get_column(above: &Vec<char>, current: &Vec<char>, below: &Vec<char>, col: usize) -> Column {
    let top = if !above.is_empty() { above[col] } else { '.' };
    let middle = current[col];
    let bottom = if !below.is_empty() { below[col] } else { '.' };
    Column {
        top,
        middle,
        bottom,
    }
}

struct Window {
    deque: VecDeque<Column>,
}

impl Window {
    fn new() -> Self {
        Self {
            deque: VecDeque::with_capacity(3),
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

    fn get_middle_value(&self) -> char {
        if let Some(middle_column) = self.deque.get(1) {
            middle_column.middle
        } else {
            '.'
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
        window.add_column(Column {
            top: 'A',
            middle: 'B',
            bottom: '.',
        });
        window.add_column(Column {
            top: 'B',
            middle: 'C',
            bottom: 'D',
        });
        window.add_column(Column {
            top: '.',
            middle: '.',
            bottom: 'E',
        });
        assert_eq!(window.count_occupied(), 5);
    }
}
