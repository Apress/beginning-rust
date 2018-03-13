/* It prints:
1 4 9 16 25 36 49 64 81 100 */
fn main() {
    let mut i = 1;
    while i <= 10 {
        print!("{} ", i * i);
        i += 1;
    }
}
