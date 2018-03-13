// ILLEGAL
fn main() {
    trait Tr {}
    let _a: &Tr = &true;
}
