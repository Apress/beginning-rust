// ILLEGAL
fn main() {
    let v1 = vec![11u8, 22];
    let result;
    {
        let v2 = vec![33u8];
        result = {
            let _x1: &Vec<u8> = &v1;
            let _x2: &Vec<u8> = &v2;
            _x2
        }
    }
    print!("{:?}", *result);
}
