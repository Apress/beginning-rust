// ILLEGAL
fn main() {
    struct S {}
    let s1 = S {};
    let s2 = s1;
    s1;
}
