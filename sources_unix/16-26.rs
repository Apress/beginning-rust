/* It prints:
a, b, c, */
fn main() {
    let arr = ['a', 'b', 'c'];
    for ch in arr.iter() {
        print!("{}, ", ch);
    }
}
