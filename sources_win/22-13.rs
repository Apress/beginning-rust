/* It prints:
12 12 13 14*/
fn main() {
    let a = 12;
    let mut b = 13;
    print!("{} ", a);
    {
        let c = &a;
        let d = &mut b;
        print!("{} {} ", c, d);
    }
    b += 1;
    print!("{}", b);
}
