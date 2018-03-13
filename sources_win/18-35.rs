/* It prints:
[10, 11, 12]; 20 21 22 23 */
fn main() {
    struct MyRangeIterator<T> {
        current: T,
        limit: T,
    }
    impl Iterator for MyRangeIterator<u32> {
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
    print!(
        "{:?}; ",
        MyRangeIterator {
            current: 10,
            limit: 13,
        }.collect::<Vec<_>>()
    );
    for i in (MyRangeIterator {
        current: 20,
        limit: 24,
    }) {
        print!("{} ", i);
    }
}
