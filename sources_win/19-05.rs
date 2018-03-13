// It does nothing.
fn main() {
    struct S1 {}
    struct S2 {}
    impl S1 {
        fn f() {}
        //ILLEGAL: fn f(a: i32) {}
    }
    impl S2 {
        fn f() {}
    }
    S1::f();
    S2::f();
}
