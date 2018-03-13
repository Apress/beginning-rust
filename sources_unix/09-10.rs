/* It prints:
3 + 5 = 8
3.2 + 5.1 = 8.3
*/
fn main() {
    {
        let addend1: f64 = 3.; let addend2: f64 = 5.;
        println!("{} + {} = {}", addend1, addend2,
            addend1 + addend2);
    }
    {
        let addend1: f64 = 3.2; let addend2: f64 = 5.1;
        println!("{} + {} = {}", addend1, addend2,
            addend1 + addend2);
    }
}
