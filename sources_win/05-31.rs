/* It prints:
[4, 56, -2] [7, 81, 12500]*/
fn main() {
    let mut a1 = [4, 56, -2];
    let a2 = [7, 81, 12500];
    print!("{:?} ", a1);
    a1 = a2;
    print!("{:?}", a1);
}
