/* It prints:
14*/
fn main() {
    trait CanBeDoubled {
        fn double(self) -> Self;
    }
    impl CanBeDoubled for i32 {
        fn double(self) -> Self {
            self * 2
        }
    }
    print!("{}", 7i32.double());
}
