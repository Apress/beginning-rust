/* It prints:
21*/
fn main() {
    S::f2();
    impl S { fn f1() { print!("1"); } }
    impl S { }
    S::f1();
    impl S { fn f2() { print!("2"); } fn _f3() {} }
    struct S {}
}
