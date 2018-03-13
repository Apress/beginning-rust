/* First, it prints:
true false
false true
4
   and then it panics.*/
fn main() {
    fn divide(numerator: f64, denominator: f64) -> Result<f64, String> {
        if denominator == 0. {
            Err(format!("Divide by zero"))
        } else {
            Ok(numerator / denominator)
        }
    }
    let r1 = divide(8., 2.);
    let r2 = divide(8., 0.);
    println!("{} {}", r1.is_ok(), r2.is_ok());
    println!("{} {}", r1.is_err(), r2.is_err());
    println!("{}", r1.unwrap());
    println!("{}", r2.unwrap());
}
