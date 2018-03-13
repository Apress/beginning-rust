/* It prints 256 lines. First:
0: []
1: []
2: []
3: []
   ... then:
65: [A]
66: [B]
67: [C]
68: [D]
   ... and at last:
253: [ý]
254: [þ]
255: [ÿ]
*/
fn main() {
    for i in 0..256 {
        println!("{}: [{}]", i, i as u8 as char);
    }
}
