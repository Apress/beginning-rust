/* It prints:
[36, 1, 15, 9, 4]*/
fn main() {
    let arr = [36, 1, 15, 9, 4];
    let v = arr.iter().collect::<Vec<_>>();
    print!("{:?}", v);
}
