/* It prints:
12 23 34 45 56 67 - B - */
fn main() {
    for item_ref in (&mut [11u8, 22, 33]).iter_mut() {
        *item_ref += 1;
        print!("{} ", *item_ref);
    }
    for item_ref in [44, 55, 66].iter_mut() {
        *item_ref += 1;
        print!("{} ", *item_ref);
    }
    for item_ref in vec!['a', 'b', 'c'].iter_mut() {
        *item_ref = if *item_ref == 'b' { 'B' } else { '-' };
        print!("{} ", *item_ref);
    }
}
