/* It prints:
10 14*/
fn main() {
    let i = 10;
    {
        let j = 4;
        {
            print!("{} ", i);
        }
        print!("{}", i + j);
    } // End of the scope of "j"
} // End of the scope of "i"
