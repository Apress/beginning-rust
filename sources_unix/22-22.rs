// ILLEGAL
fn main() {
    let mut a = 12;
    let _b = &mut a;
    let _c = &mut a;
}
