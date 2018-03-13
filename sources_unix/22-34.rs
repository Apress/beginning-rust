// It does nothing.
fn main() {
    fn func(v: &Vec<u8>) -> &u8 {
        &v[3]
    }
}
