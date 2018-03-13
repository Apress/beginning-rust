/* It prints:
0 2.3*/
fn main() {
    struct Number {
        x: f64,
    }
    impl Number {
        fn new() -> Number { Number { x: 0. } }
        fn from(x: f64) -> Number { Number { x: x } }
        fn value(&self) -> f64 { self.x }
    }
    let a = Number::new();
    let b = Number::from(2.3);
    print!("{} {}", a.value(), b.value());
}
