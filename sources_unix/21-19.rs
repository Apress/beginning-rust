// ILLEGAL
fn main() {
    let v1 = vec![false; 3];
    let mut v2 = vec![false; 2];
    v2 = v1;
    v1;
}
