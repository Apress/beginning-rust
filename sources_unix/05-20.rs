/* It prints:
This line contains a sentence */
fn main() {
    let mut x = vec!["This", "is", "a", "sentence"];
    x.insert(1, "line");
    x.insert(2, "contains");
    x.remove(3);
    x.push("about Rust");
    x.pop();
    for i in 0..x.len() { print!("{} ", x[i]); }
}
