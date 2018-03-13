// ILLEGAL
fn main() {
    fn f1() -> i32 { 4.5; "abc"; false }
    fn f2() -> i32 { 4.5; "abc"; () }
    fn f3() -> i32 { 4.5; "abc"; {} }
    fn f4() -> i32 { 4.5; "abc"; }
}
