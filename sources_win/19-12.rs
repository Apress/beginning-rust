/* It prints:
12 17 */
fn main() {
    struct S { x: u32 }
    impl S {
        fn get_x(&self) -> u32 { self.x }
        fn set_x(&mut self, x: u32) { self.x = x; }
    }
    let mut s = S { x: 12 };
    print!("{} ", s.get_x());
    s.set_x(17);
    print!("{} ", s.get_x());
}
