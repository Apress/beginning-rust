// ILLEGAL
fn main() {
    struct S {
        _b: bool,
        _ri: &i32,
    }
    let _y: S;
    let x: i32 = 12;
    _y = S { _b: true, _ri: &x };
}
