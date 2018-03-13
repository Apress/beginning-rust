// ILLEGAL
fn main() {
    struct S { x: Vec<i32> }
    impl Copy for S {}
    impl Clone for S {
        fn clone(&self) -> Self { *self }
    }
}
