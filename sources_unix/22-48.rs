// ILLEGAL
fn main() {
    let v1 = vec![11u8, 22];
    let result;
    {
        let v2 = vec![33u8];
        fn func<'a>(_x1: &Vec<u8>, _x2: &'a Vec<u8>) -> &'a Vec<u8> {
            _x2
        }
        result = func(&v1, &v2);
    }
    print!("{:?}", *result);
}
