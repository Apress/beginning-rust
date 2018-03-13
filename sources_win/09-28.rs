/* It prints:
[5, -8, 9, 0, -14, -2, 3, 5, 3, 1]*/
fn main() {
    fn double_negatives(mut a: [i32; 10]) -> [i32; 10] {
        for i in 0..10 {
            if a[i] < 0 { a[i] *= 2; }
        }
        a
    }
    let mut arr = [5, -4, 9, 0, -7, -1, 3, 5, 3, 1];
    arr = double_negatives(arr);
    print!("{:?}", arr);
}
