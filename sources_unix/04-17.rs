/* It prints:
1 2 3 :1*/
fn main() {
    let mut limit = 4;
    for i in 1..limit {
        limit -= 1;
        print!("{} ", i);
    }
    print!(":{}", limit);
}
