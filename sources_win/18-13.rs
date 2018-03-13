/* It prints:
3.1622776601683795 3.1622777*/
fn main() {
    trait HasSquareRoot {
        fn sqrt(self) -> Self;
    }
    impl HasSquareRoot for f32 {
        fn sqrt(self) -> Self { f32::sqrt(self) }
    }
    impl HasSquareRoot for f64 {
        fn sqrt(self) -> Self { f64::sqrt(self) }
    }
    trait HasAbsoluteValue {
        fn abs(self) -> Self;
    }
    impl HasAbsoluteValue for f32 {
        fn abs(self) -> Self { f32::abs(self) }
    }
    impl HasAbsoluteValue for f64 {
        fn abs(self) -> Self { f64::abs(self) }
    }
    fn abs_quartic_root<Number>(x: Number) -> Number
    where Number: HasSquareRoot + HasAbsoluteValue {
        x.abs().sqrt().sqrt()
    }
    print!("{} {}",
        abs_quartic_root(-100f64),
        abs_quartic_root(-100f32));
}
