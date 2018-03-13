// It does nothing.
fn main() {
    trait Tr {
        fn f<'a>(flag: bool, b: &'a i32, c: (char, &i32))
            -> (&'static i32, f64, &'a i32);
    }
}
