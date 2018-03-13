/* It prints:
0, 1; false true*/
fn main() {
    trait Searchable<Key, Count> {
        fn contains(&self, key: Key) -> bool;
        fn count(&self, key: Key) -> Count;
    }
    struct RecordWithId {
        id: u32,
        _descr: String,
    }
    struct NameSetWithId {
        data: Vec<RecordWithId>,
    }
    impl Searchable<u32, usize> for NameSetWithId {
        fn contains(&self, key: u32) -> bool {
            for record in self.data.iter() {
                if record.id == key {
                    return true;
                }
            }
            false
        }
        fn count(&self, key: u32) -> usize {
            let mut c = 0;
            for record in self.data.iter() {
                if record.id == key {
                    c += 1;
                }
            }
            c
        }
    }
    fn is_present<Collection>(coll: &Collection, id: u32) -> bool
    where
        Collection: Searchable<u32, usize>,
    {
        coll.contains(id)
    }
    let names = NameSetWithId {
        data: vec![
            RecordWithId {
                id: 34,
                _descr: "John".to_string(),
            },
            RecordWithId {
                id: 49,
                _descr: "Jane".to_string(),
            },
        ],
    };
    print!(
        "{}, {}; {} {}",
        names.count(48),
        names.count(49),
        is_present(&names, 48),
        is_present(&names, 49)
    );
}
