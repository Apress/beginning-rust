/* It prints:
Some(10), Some(11), Some(12), None, None, */
fn main() {
    trait MyIterator {
        type Item;
        fn next(&mut self) -> Option<Self::Item>;
    }
    struct MyRangeIterator<T> {
        current: T,
        limit: T,
    }
    impl MyIterator for MyRangeIterator<u32> {
        type Item = u32;
        fn next(&mut self) -> Option<Self::Item> {
            if self.current == self.limit {
                None
            } else {
                self.current += 1;
                Some(self.current - 1)
            }
        }
    }
    let mut range_it = MyRangeIterator {
        current: 10,
        limit: 13,
    };
    print!("{:?}, ", range_it.next());
    print!("{:?}, ", range_it.next());
    print!("{:?}, ", range_it.next());
    print!("{:?}, ", range_it.next());
    print!("{:?}, ", range_it.next());
}
