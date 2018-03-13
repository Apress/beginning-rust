/* It prints:
1, 1, 2, 3, 5, 8, 13, 21, 34, 55, 89, 144, 233, 377, 610, */
fn main() {
    let mut fib = [1; 15];
    for i in 2..fib.len() {
        fib[i] = fib[i - 2] + fib[i - 1];
    }
    for i in 0..fib.len() {
        print!("{}, ", fib[i]);
    }
}
