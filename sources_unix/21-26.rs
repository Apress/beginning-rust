// It does nothing.
fn main() {
    struct S {}
    impl Clone for S {
        fn clone(&self) -> Self { Self {} }
    }
    let s = S {};
    s.clone();
}
