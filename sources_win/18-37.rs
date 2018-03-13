/* It prints:
None Some(23) None Some(44)*/
fn main() {
    print!(
        "{:?} {:?} {:?} {:?}",
        (10..12).nth(2),
        (20..29).nth(2),
        ([31, 32].iter()).nth(2),
        ([41, 42, 43, 44].iter()).nth(2)
    );
}
