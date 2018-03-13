/* It prints:
[5, -8, 9, 0, -14, -2, 3, 5, 3, 1]*/
fn main() {
    fn double_negatives(a: &mut [i32; 10]) {
        for i in 0..10 {
            if a[i] < 0 { a[i] *= 2; }
        }
    }
    let mut arr = [5, -4, 9, 0, -7, -1, 3, 5, 3, 1];
    double_negatives(&mut arr);
    print!("{:?}", arr);
}
