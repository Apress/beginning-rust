/* It prints:
[5, -8, 9, 0, -14, -2, 3, 5, 3, 1]*/
fn main() {
    let mut arr = [5, -4, 9, 0, -7, -1, 3, 5, 3, 1];
    for i in 0..10 {
        if arr[i] < 0 { arr[i] *= 2; }
    }
    print!("{:?}", arr);
}
