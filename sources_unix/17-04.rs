// It prints the list of current environment variables.
fn main() {
    for var in std::env::vars() {
        println!("[{}]=[{}]", var.0, var.1);
    }
}
