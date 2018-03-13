/* It prints:
100.00000000000003 3.1622776601683795*/
fn main() {
    fn quartic_root(x: f64) -> f64 { x.sqrt().sqrt() }
    let qr = quartic_root(100f64);
    print!("{} {}", qr * qr * qr * qr, qr);
}
