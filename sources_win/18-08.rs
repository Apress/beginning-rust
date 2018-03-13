// ILLEGAL
fn main() {
    fn g(a: i32) { }
    fn f<T>(a: T) -> bool {
        g(a);
        a == a
    }
}
