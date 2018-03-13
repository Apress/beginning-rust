// ILLEGAL
fn main() {
    let mut a1 = [4, 56, -2];
    let a2 = [7, 81];
    print!("{:?} ", a1);
    a1 = a2;
    print!("{:?}", a1);
}
