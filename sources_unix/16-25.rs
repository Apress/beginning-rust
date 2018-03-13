/* It prints:
0 a, 1 b, 2 c, */
fn main() {
    let arr = ['a', 'b', 'c'];
    for i in 0..arr.len() {
        print!("{} {}, ", i, arr[i]);
    }
}
