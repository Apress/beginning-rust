// ILLEGAL
fn main() {
    fn f(v2: Vec<bool>) {}
    let v1 = vec![false; 3];
    f(v1);
    v1;
}
