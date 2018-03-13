// ILLEGAL
fn main() {
    let a: str;
    fn f(a: str) {}
    print!("{}", std::mem::size_of::<str>());
}
