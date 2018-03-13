// It does nothing.
fn main() {
    [66, -8, 43, 19, 0, -31]
        .iter()
        .filter(|x| { print!("F{} ", x); **x > 0 })
        .map(|x| { print!("M{} ", x); *x * 2 });
}
