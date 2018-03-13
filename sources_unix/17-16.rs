/* If in the current directory there is a readable file named "data.txt",
   this program prints the contents of such file.
   In any other case, it panics.
*/
fn main() {
    use std::io::Read;
    let mut file = std::fs::File::open("data.txt").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    print!("{}", contents);
}
