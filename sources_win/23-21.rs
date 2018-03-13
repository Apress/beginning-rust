// ILLEGAL
fn main() {
    struct S<'a, T> { b: &'a T }
}
