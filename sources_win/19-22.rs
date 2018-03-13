// It does nothing.
fn main() {
    trait Tr {}
    impl Tr for bool {}
    let _a: &Tr = &true;
}
