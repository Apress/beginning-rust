/* It prints:
16 10 8 2*/
fn main() {
    let hexadecimal = 0x10;
    let decimal = 10;
    let octal = 0o10;
    let binary = 0b10;
    print!("{} {} {} {}",
        hexadecimal, decimal, octal, binary);
}
