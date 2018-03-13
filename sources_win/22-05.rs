// ILLEGAL
fn main() {
    let ref_to_n;
    {
        let n = 12;
        ref_to_n = &n;
        print!("{} ", *ref_to_n);
    }
    print!("{}", *ref_to_n);
}
