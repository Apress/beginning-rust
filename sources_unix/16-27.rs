/* It prints:
0 a, 1 b, 2 c, */
fn main() {
    let arr = ['a', 'b', 'c'];
    let mut i = 0;
    for ch in arr.iter() {
        print!("{} {}, ", i, *ch);
        i += 1;
    }
}
