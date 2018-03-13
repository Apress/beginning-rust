/* It prints:
12 23 34 45 56 67 - B - */
fn main() {
    let slice: &mut [u8] = &mut [11u8, 22, 33];
    let slice_it: std::slice::IterMut<u8> = slice.iter_mut();
    for item_ref in slice_it {
        *item_ref += 1;
        print!("{} ", *item_ref);
    }
    let mut arr: [i32; 3] = [44, 55, 66];
    let arr_it: std::slice::IterMut<i32> = arr.iter_mut();
    for item_ref in arr_it {
        *item_ref += 1;
        print!("{} ", *item_ref);
    }
    let mut vec: Vec<char> = vec!['a', 'b', 'c'];
    let vec_it: std::slice::IterMut<char> = vec.iter_mut();
    for item_ref in vec_it {
        *item_ref = if *item_ref == 'b' { 'B' } else { '-' };
        print!("{} ", *item_ref);
    }
}
