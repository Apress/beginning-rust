/* It prints:
None Some(3.1)*/
fn main() {
    fn get_third(s: &[f64]) -> Option<f64> {
        if s.len() >= 3 {
            Some(s[2])
        } else {
            None
        }
    }
    print!("{:?} {:?}",
        get_third(&[1.0, 2.0]),
        get_third(&[1.1, 2.1, 3.1]));
}
