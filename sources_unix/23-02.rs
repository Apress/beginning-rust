// It does nothing.
fn main() {
    trait Tr {
        fn f<'a>(x: &'a u8) -> &'a u8;
    }
}
