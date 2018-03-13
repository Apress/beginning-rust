// It does nothing.

// In some library code:
struct S {
    _b: bool,
    _ri: &'static i32,
}
fn create_s(ri: &i32) -> S {
    static ZERO: i32 = 0;
    static ONE: i32 = 1;
    S {
        _b: true,
        _ri: if *ri > 0 { &ONE } else { &ZERO },
    }
}

// In application code:
fn main() {
    let _y: S;
    let x: i32 = 12;
    _y = create_s(&x);
}
