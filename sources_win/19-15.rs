/* It prints:
() (abcd); [] [97, 98, 99, 100]*/
fn main() {
    let a = String::new();
    let b = String::from("abcd");
    print!("({}) ({});", a, b);
    let c = Vec::<i32>::new();
    let d = Vec::<u8>::from("abcd");
    print!(" {:?} {:?}", c, d);
}
