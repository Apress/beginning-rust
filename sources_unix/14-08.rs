/* It prints:
0x1 0 0
<an exedecimal number> 4 1
<an exedecimal number> 4 2
<an exedecimal number> 4 3
<an exedecimal number> 4 4
<an exedecimal number> 8 5
<an exedecimal number> 8 6
<an exedecimal number> 8 7
<an exedecimal number> 8 8
<an exedecimal number> 16 9
<an exedecimal number> 16 10: aaaaaaaaaa
*/
fn main() {
    let mut s = "".to_string();
    for _ in 0..10 {
        println!("{:?} {} {}",
            s.as_ptr(), s.capacity(), s.len());
        s.push('a');
    }
    println!("{:?} {} {}: {}",
        s.as_ptr(), s.capacity(), s.len(), s);
}
