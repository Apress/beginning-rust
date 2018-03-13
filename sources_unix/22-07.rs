// ILLEGAL
fn main() {
    let mut v = vec![12];
    let ref_to_first = &v[0];
    v.push(13);
    print!("{}", ref_to_first);
}
