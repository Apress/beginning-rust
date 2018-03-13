/* It prints:
a X; b X; c X; */
fn main() {
    let mut r = "abc".chars();
    for i in r {
        r = "XY".chars();
        print!("{} {}; ", i, r.next().unwrap());
    }
}
