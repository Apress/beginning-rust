/* It prints:
3.14*/
fn main() {
    fn f<'a, T>(b: &'a T) -> &'a T { b }
    let pi = 3.14;
    let pi_ref = f(&pi);
    print!("{}", *pi_ref);
}
