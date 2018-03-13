/* It prints:
Hello, world!*/
fn main() {
    let mut dyn_str = "Hello".to_string();
    dyn_str.push_str(", ");
    dyn_str.push_str("world");
    dyn_str.push_str("!");
    print!("{}", dyn_str);
}
