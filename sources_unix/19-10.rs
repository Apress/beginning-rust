/* It prints:
Tr::f1 Tr::f2 */
fn main() {
    trait Tr {
        fn f1();
        fn f2(&self);
    }
    impl Tr for bool {
        fn f1() { print!("Tr::f1 "); }
        fn f2(&self) { print!("Tr::f2 "); }
    }
    bool::f1();
    true.f2();
}
