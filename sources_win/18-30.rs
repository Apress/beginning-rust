/* It prints:
false true*/
fn main() {
    trait Searchable<Key> {
        fn contains(&self, key: Key) -> bool;
    }
    struct RecordWithId {
        id: u32,
        _descr: String,
    }
    struct NameSetWithId {
        data: Vec<RecordWithId>,
    }
    impl Searchable<u32> for NameSetWithId {
        fn contains(&self, key: u32) -> bool {
            for record in self.data.iter() {
                if record.id == key {
                    return true;
                }
            }
            false
        }
    }
    fn is_present<Collection>(coll: &Collection, id: u32) -> bool
    where
        Collection: Searchable<u32>,
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
    print!("{} {}", is_present(&names, 48), is_present(&names, 49));
}
