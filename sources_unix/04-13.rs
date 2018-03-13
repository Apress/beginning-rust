/* It prints:
1 4 9 16 25 36 49 64 81 100 121 144 169 196 */
fn main() {
    let mut i = 1;
    while true {
        let ii = i * i;
        if ii >= 200 { break; }
        print!("{} ", ii);
        i += 1;
    }
}
