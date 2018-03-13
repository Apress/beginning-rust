/* It prints:
false true */
fn main() {
    print!("{} ",
        [45, 8, 2, 6].iter().any(|n| *n < 0));
    print!("{} ",
        [45, 8, -2, 6].iter().any(|n| *n < 0));
}
