/* It prints:
[12, 23, 34] [45, 56, 67] ['-', 'B', '-'] */
fn main() {
    let slice = &mut [11u8, 22, 33];
    for item_ref in slice.iter_mut() {
        *item_ref += 1;
    }
    print!("{:?} ", slice);

    let mut arr = [44, 55, 66];
    for item_ref in arr.iter_mut() {
        *item_ref += 1;
    }
    print!("{:?} ", arr);

    let mut vec = vec!['a', 'b', 'c'];
    for item_ref in vec.iter_mut() {
        *item_ref = if *item_ref == 'b' { 'B' } else { '-' };
    }
    print!("{:?} ", vec);
}
