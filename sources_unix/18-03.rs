// ILLEGAL
fn main() {
    fn quartic_root<Number>(x: Number) -> Number {
        x.sqrt().sqrt()
    }
    print!("{} {}",
        quartic_root(100f64),
        quartic_root(100f32));
}
