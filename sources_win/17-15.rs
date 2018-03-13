/* It does not print anything, but creates a file named "data.txt"
   in the current directory, and sets its contents to "eè€".*/
fn main() {
    use std::io::Write;
    let mut file = std::fs::File::create("data.txt").unwrap();
    file.write_all("eè€".as_bytes()).unwrap();
}
