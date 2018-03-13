/* It prints:
11 22 33 44 55 66 a b c */
fn main() {
    let slice: &[u8] = &[11u8, 22, 33];
    let slice_it: std::slice::Iter<u8> = slice.iter();
    for item_ref in slice_it {
        // *item_ref += 1;
        print!("{} ", *item_ref);
    }
    let arr: [i32; 3] = [44, 55, 66];
    let arr_it: std::slice::Iter<i32> = arr.iter();
    for item_ref in arr_it {
        // *item_ref += 1;
        print!("{} ", *item_ref);
    }
    let vec: Vec<char> = vec!['a', 'b', 'c'];
    let vec_it: std::slice::Iter<char> = vec.iter();
    for item_ref in vec_it {
        // *item_ref = if *item_ref == 'b' { 'B' } else { '-' };
        print!("{} ", *item_ref);
    }
}
