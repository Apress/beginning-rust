// ILLEGAL
fn main() {
    fn get_third<Iter, Item>(mut iterator: Iter) -> Option<Item> {
        iterator.next();
        iterator.next();
        iterator.next()
    }
    print!("{:?} {:?}",
        get_third(0..9),
        get_third([11, 22, 33, 44].iter()));
}
