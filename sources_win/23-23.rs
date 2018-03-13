/* It prints:
true true*/
fn main() {
    struct S<'a, T: 'a> { b: &'a T }
    let s1 = S { b: &true };
    let s2 = S { b: &s1 };
    print!("{} {}", *s1.b, *s2.b.b);
}
