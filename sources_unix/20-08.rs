/* It prints:
48 35 20 39 42 33 50 29 27 18 17 16 13 12 */
fn main() {
    fn add(v: &mut Vec<i32>, a: i32) {
        v.push(a);
        v.sort();
    }
    let a = [48, 18, 20, 35, 17, 13, 39,
        12, 42, 33, 29, 27, 50, 16];
    let mut v = Vec::<i32>::new();
    for i in 0..a.len() / 2 {
        add(&mut v, a[i * 2]);
        add(&mut v, a[i * 2 + 1]);
        print!("{} ", v.pop().unwrap());
    }
    while ! v.is_empty() {
        print!("{} ", v.pop().unwrap());
    }
}
