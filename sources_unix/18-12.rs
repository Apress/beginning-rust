/* It prints:
3.1622776601683795 3.1622777*/
fn main() {
    trait HasSqrtAndAbs {
        fn sqrt(self) -> Self;
        fn abs(self) -> Self;
    }
    impl HasSqrtAndAbs for f32 {
        fn sqrt(self) -> Self { f32::sqrt(self) }
        fn abs(self) -> Self { f32::abs(self) }
    }
    impl HasSqrtAndAbs for f64 {
        fn sqrt(self) -> Self { f64::sqrt(self) }
        fn abs(self) -> Self { f64::abs(self) }
    }
    fn abs_quartic_root<Number>(x: Number) -> Number
    where Number: HasSqrtAndAbs {
        x.abs().sqrt().sqrt()
    }
    print!("{} {}",
        abs_quartic_root(-100f64),
        abs_quartic_root(-100f32));
}
