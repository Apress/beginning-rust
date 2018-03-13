/* It prints:
[5, -4, 9, 0, -7, -1, 3, 5, 3, 1]*/
fn main() {
    fn double_negatives(mut a: [i32; 10]) {
        for i in 0..10 {
            if a[i] < 0 { a[i] *= 2; }
        }
    }
    let mut arr = [5, -4, 9, 0, -7, -1, 3, 5, 3, 1];
    double_negatives(arr);
    print!("{:?}", arr);
}
