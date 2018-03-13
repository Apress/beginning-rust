/* It prints:
48 35 20 39 42 33 50 29 27 18 17 16 13 12 */
fn main() {
    let a = [48, 18, 20, 35, 17, 13, 39,
        12, 42, 33, 29, 27, 50, 16];
    let mut v = std::collections::BinaryHeap::<i32>::new();
    for i in 0..a.len() / 2 {
        v.push(a[i * 2]);
        v.push(a[i * 2 + 1]);
        print!("{} ", v.pop().unwrap());
    }
    while ! v.is_empty() {
        print!("{} ", v.pop().unwrap());
    }
}
