// ILLEGAL
fn main() {
    let v1 = vec![false; 0];
    let mut v2 = vec![false; 0];
    v2 = v1;
    v1;
}
