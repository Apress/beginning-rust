// ILLEGAL

// In some library code:
struct S {
    _b: bool,
    _ri: &i32,
}
fn create_s(ri: &i32) -> S {
    S { _b: true, _ri: ri }
}

// In application code:
fn main() {
    let _y: S;
    let x: i32 = 12;
    _y = create_s(&x);
}
