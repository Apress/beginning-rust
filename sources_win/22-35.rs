// ILLEGAL
fn main() {
    trait Tr {
        fn f(flag: bool, b: &i32, c: (char, &i32)) -> (&i32, f64, &i32);
    }
}
