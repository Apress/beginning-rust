/* It prints:
10 11 12 13 14 */
fn main() {
    let r = 0..5;
    for mut i in r {
        i += 10;
        print!("{} ", i);
    }
}
