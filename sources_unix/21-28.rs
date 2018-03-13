// It does nothing.
fn main() {
    struct S {}
    impl Clone for S {
        fn clone(&self) -> Self { Self {} }
    }
    impl Copy for S {}
    let s = S {};
    s.clone();
    let _s2 = s;
    s;
}
