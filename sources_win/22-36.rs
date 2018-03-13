// It does nothing.
fn main() {
    trait Tr {
        fn f<'a>(flag: bool, b: &'a i32, c: (char, &'a i32))
            -> (&'a i32, f64, &'static i32);
    }
}
