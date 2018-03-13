// It prints the path used to run the program,
// and all its command-line arguments,
// each one enclosed in square brackets.
fn main() {
    let command_line: std::env::Args = std::env::args();
    for argument in command_line {
        println!("[{}]", argument);
    }
}
