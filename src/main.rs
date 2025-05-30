mod expression;
mod globals;
mod polynomial;
mod reverse_polish_calculator;

use std::io::BufRead;

fn main() {
    let stdin = std::io::stdin();
    let reader = stdin.lock();

    // read the standard input and then output
    for line in reader.lines() {
        let line = line.unwrap();

        let result = reverse_polish_calculator::calculate(line);
        match result {
            Ok(output) => {
                //
                println!("{}", output)
            }
            Err(err) => println!("{}", err.to_string()),
        }
    }
}
