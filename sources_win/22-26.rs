// It does nothing.
fn main() {
    let mut a = 12;
    {
        let b = &mut a;
        *b += 1;
    }
    let c = &mut a;
    *c += 2;
}
