/* It prints:
true*/
fn main() {
    struct S<'a, T: 'static> { b: &'a T }
    let s = S { b: &true };
    print!("{}", *s.b);
}
