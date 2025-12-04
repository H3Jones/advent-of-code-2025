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
        line_above_buffer = if row > 0 { current_line_buffer.clone() } else { Vec::new() };
        current_line_buffer = output[row].clone();
        //last row special case
        next_line_buffer = if row + 1 < output.len() { output[row + 1].clone() } else { Vec::new() };

        println!(
            "Above: {:?}\nCurrent: {:?}\nBelow: {:?}\n",
            line_above_buffer,
            current_line_buffer,
            next_line_buffer
        );
    }
}


