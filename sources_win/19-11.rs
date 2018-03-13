/* It prints:
None*/
fn main() {
    struct Pair(u32, u32);
    impl std::iter::Iterator for Pair {
        type Item = u32;
        fn next(&mut self) -> Option<Self::Item> {
            None
        }
    }
    let mut a = Pair(23u32, 34u32);
    print!("{:?}", a.next());
}
