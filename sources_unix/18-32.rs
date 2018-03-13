/* It prints:
0, 1; false true*/
fn main() {
    trait Searchable { //1
        type Key; //2
        type Count; //3
        fn contains(&self, key: Self::Key) -> bool; //4
        fn count(&self, key: Self::Key) -> Self::Count; //5
    }
    struct RecordWithId {
        id: u32,
        _descr: String,
    }
    struct NameSetWithId {
        data: Vec<RecordWithId>,
    }
    impl Searchable for NameSetWithId { //6
        type Key = u32; //7
        type Count = usize; //8
        fn contains(&self, key: Self::Key) -> bool { //9
            for record in self.data.iter() {
                if record.id == key {
                    return true;
                }
            }
            false
        }
        fn count(&self, key: Self::Key) -> usize { //10
            let mut c = 0;
            for record in self.data.iter() {
                if record.id == key {
                    c += 1;
                }
            }
            c
        }
    }
    fn is_present<Collection>(
        coll: &Collection,
        id: <Collection as Searchable>::Key,//11
    ) -> bool
    where
        Collection: Searchable, //12
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
