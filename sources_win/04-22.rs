/* It prints:
4 10 */
fn main() {
    let i = 10;
    {
        let i = 4;
        print!("{} ", i);
    } // End of the scope of the second "i"
    print!("{} ", i);
} // End of the scope of the first "i"
