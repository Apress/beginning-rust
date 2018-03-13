// ILLEGAL
fn main() {
    fn f(a: i16) {}
    f(3.);
    f(3u16);
    f(3i16);
    f(3);
}
