// It does nothing.
fn main() {
    let mut a = 12;
    fn f(b: &mut i32) {
        *b += 1;
    }
    f(&mut a);
    let c = &mut a;
    *c += 2;
}
