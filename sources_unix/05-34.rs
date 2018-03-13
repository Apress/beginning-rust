/* It prints:
[4, 56, -2] [7, 81]*/
fn main() {
    let mut a1 = vec![4, 56, -2];
    let a2 = vec![7, 81];
    print!("{:?} ", a1);
    a1 = a2;
    print!("{:?}", a1);
}
