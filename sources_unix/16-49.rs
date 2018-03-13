/* It prints:
F66 M66 F-8 F43 M43 F19 M19 F0 F-31 [132, 86, 38]*/
fn main() {
    let v = [66, -8, 43, 19, 0, -31]
        .iter()
        .filter(|x| { print!("F{} ", x); **x > 0 })
        .map(|x| { print!("M{} ", x); *x * 2 })
        .collect::<Vec<_>>();
    print!("{:?}", v);
}
