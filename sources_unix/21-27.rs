// ILLEGAL
fn main() {
    struct S {}
    impl Clone for S {
        fn clone(&self) -> Self { Self {} }
    }
    let s = S {};
    s.clone();
    let _s2 = s;
    s;
}
