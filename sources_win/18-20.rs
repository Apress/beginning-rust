/* It prints:
0 0 3 2 */
fn main() {
    trait LettersCount {
        fn letters_count(&self, ch: char) -> usize;
    }
    impl LettersCount for str {
        fn letters_count(&self, ch: char) -> usize {
            self.chars().filter(|c| *c == ch).count()
        }
    }
    print!("{} ", "".letters_count('a'));
    print!("{} ", "ddd".letters_count('a'));
    print!("{} ", "ddd".letters_count('d'));
    print!("{} ", "foobarbaz".letters_count('a'));
}
