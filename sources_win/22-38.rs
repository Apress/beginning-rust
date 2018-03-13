// It does nothing.
fn main() {
    trait Tr {
        fn f<'a, 'b, T1, T2>(flag: bool, b: &'a T1, c: (char, &'b i32))
            -> (&'b i32, f64, &'a T2);
    }
}
