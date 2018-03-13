/* It prints:
Hello, world!*/
fn main() {
    let comma = ", ".to_string();
    let world = "world".to_string();
    let excl_point = '!';
    let mut dyn_str = "Hello".to_string();
    dyn_str += &comma;
    dyn_str.push_str(&world);
    dyn_str.push(excl_point);
    print!("{}", dyn_str);
}
