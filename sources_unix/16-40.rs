/* It prints:
brave world */
fn main() {
    let arr = ["hello", "brave", "new", "world"];
    match arr.iter().min() {
        Some(n) => print!("{} ", n),
        _ => (),
    }
    match arr.iter().max() {
        Some(n) => print!("{} ", n),
        _ => (),
    }
}
