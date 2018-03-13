// It does nothing.
fn main() {
    fn _f1<T>(a: T) -> T { a }
    fn _f2<T>(a: T) -> T {
        let b: T = a;
        let mut c = b;
        c = _f1(c);
        c
    }
    fn _f3<T>(a: &T) -> &T { a }
}
