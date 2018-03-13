/* It prints:
"Hello"
['H', 'e', 'l', 'l', 'o']
[72, 101, 108, 108, 111]
[72, 101, 108, 108, 111]
*/
fn main() {
    let s = "Hello";
    println!("{:?}", s.chars().collect::<String>());
    println!("{:?}", s.chars().collect::<Vec<char>>());
    println!("{:?}", s.bytes().collect::<Vec<u8>>());
    println!("{:?}", s.as_bytes().iter().collect::<Vec<&u8>>());
}
