/* It waits for two lines typed from the console.
   After the user has typed such lines, it prints:
First: <first line>
Second: <second line>
: <integer number> bytes
*/
fn main() {
    let mut text = format!("First: ");
    let inp = std::io::stdin();
    inp.read_line(&mut text);
    text.push_str("Second: ");
    inp.read_line(&mut text);
    println!("{}: {} bytes", text, text.len());
}
