/* It prints:
[4, 5, 6]*/
fn main() {
    let slice = &mut [3, 4, 5];
    {
        let iterator: std::slice::IterMut<i32> =
            slice.iter_mut();
        for item_ref in iterator {
            *item_ref += 1;
        }
    }
    print!("{:?}", slice);
}
