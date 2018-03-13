// ILLEGAL
fn main() {
    let mut slice = &mut [3, 4, 5];
    {
        let mut iterator = slice.iter();
        for mut item_ref in iterator {
            *item_ref += 1;
        }
    }
    print!("{:?}", slice);
}
