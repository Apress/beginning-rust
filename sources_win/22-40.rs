// ILLEGAL
fn main() {
    fn f(n: &u8) -> &'static u8 {
        n
    }
    print!("{}", *f(&12));
}
