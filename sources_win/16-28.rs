/* It prints:
0 a, 1 b, 2 c, */
fn main() {
    let arr = ['a', 'b', 'c'];
    for (i, ch) in arr.iter().enumerate() {
        print!("{} {}, ", i, *ch);
    }
}
