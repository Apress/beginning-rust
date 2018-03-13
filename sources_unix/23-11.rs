// ILLEGAL
fn main() {
    struct S {
        _b: bool,
        _ri: &i32,
    }
    let x: i32 = 12;
    let _y: S = S { _b: true, _ri: &x };
}
