// It does nothing.

// In some library code:
struct S<'a> { _b: bool, _ri: &'a i32 }
fn create_s<'b>(ri: &'b i32) -> S<'b> {
    S { _b: true, _ri: ri }
}

// In application code:
fn main() {
    let x: i32 = 12;
    let _y: S;
    _y = create_s(&x);
}
