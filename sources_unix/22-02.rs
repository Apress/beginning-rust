// ILLEGAL
fn main() {
    let mut n = 12;
    let ref1_to_n = &mut n;
    let ref2_to_n = &n;
}
