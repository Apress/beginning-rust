// ILLEGAL
fn main() {
    fn f() -> i32 { 3 }
    let _a: u32 = f();
}
