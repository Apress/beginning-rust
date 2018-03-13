/* It prints:
3.1622776601683795 3.1622777*/
fn main() {
    trait HasSquareRoot {
        fn sq_root(self) -> Self;
    }
    impl HasSquareRoot for f32 {
        fn sq_root(self) -> Self { f32::sqrt(self) }
    }
    impl HasSquareRoot for f64 {
        fn sq_root(self) -> Self { f64::sqrt(self) }
    }
    fn quartic_root<Number>(x: Number) -> Number
    where Number: HasSquareRoot {
        x.sq_root().sq_root()
    }
    print!("{} {}",
        quartic_root(100f64),
        quartic_root(100f32));
}
