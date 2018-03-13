/* It prints:
bye bye*/
fn main() {
    let word = "bye".to_string();
    let w1: &str = &word;
    let w2: &String = &word;
    print!("{} {}", w1, w2);
}
