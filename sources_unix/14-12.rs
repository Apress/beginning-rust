/* It prints:
Hello, world!*/
fn main() {
    let mut dyn_str = "Hello".to_string();
    dyn_str = format!("{}{}", dyn_str, ", ");
    dyn_str = format!("{}{}", dyn_str, "world");
    dyn_str = format!("{}{}", dyn_str, "!");
    print!("{}", dyn_str);
}
