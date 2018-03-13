/* It prints:
None Some(22) None Some(43)*/
fn main() {
    fn get_third<Iter>(mut iterator: Iter) -> Option<Iter::Item>
    where
        Iter: std::iter::Iterator,
    {
        iterator.next();
        iterator.next();
        iterator.next()
    }
    print!(
        "{:?} {:?} {:?} {:?}",
        get_third(10..12),
        get_third(20..29),
        get_third([31, 32].iter()),
        get_third([41, 42, 43, 44].iter())
    );
}
