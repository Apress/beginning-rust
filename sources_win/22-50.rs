// ILLEGAL
fn main() {
    fn f<'a>(x: &'a i32, y: &'a i32) -> (&'a i32, bool, &'a i32) {
        (x, true, y)
    }
    let i1 = 12;
    let i2;
    let j1 = 13;
    let j2;
    let r = f(&i1, &j1);
    i2 = r.0;
    j2 = r.2;
    print!("{} {} {}", *i2, r.1, *j2);
}
