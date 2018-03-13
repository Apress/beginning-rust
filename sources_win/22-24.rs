// ILLEGAL
fn main() {
    let mut a = 12;
    let _b = &mut a;
    a = 13;
}
