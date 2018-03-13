/* It prints:
1 4 16 25 49 64 100 121 169 196 256 289 361 400 */
fn main() {
    let mut i = 0;
    while i < 50 {
        i += 1;
        if i % 3 != 0 {
            if i * i <= 400 {
                print!("{} ", i * i);
            }
        }
    }
}
