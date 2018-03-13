// ILLEGAL
fn main() {
    enum T {A, B, C, D};
    let n: i32 = T::D;
    let e: T = 1;
}
