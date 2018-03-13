// It prints the path used to run the program,
// and all its command-line arguments,
// each one enclosed in square brackets.
fn main() {
    for a in std::env::args() {
        println!("[{}]", a);
    }
}
