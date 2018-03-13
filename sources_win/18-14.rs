/* It prints:
abcd,3,[7]; abcd,3,[7]*/
fn main() {
    print!("{},", "abcd".to_string());
    print!("{},", [1, 2, 3].len());
    let mut v1 = vec![0u8; 0];
    v1.push(7u8);
    print!("{:?}; ", v1);

    print!("{},", std::string::ToString::to_string("abcd"));
    print!("{:?},", <[i32]>::len(&[1, 2, 3]));
    let mut v2 = vec![0u8; 0];
    Vec::push(&mut v2, 7u8);
    print!("{:?}", v2);
}
