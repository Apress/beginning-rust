/* It prints:
0 0 3 2 */
fn main() {
    trait LettersCount {
        fn letters_count(&self, ch: char) -> usize;
    }
    impl LettersCount for str {
        fn letters_count(&self, ch: char) -> usize {
            let mut count = 0;
            for c in self.chars() {
                if c == ch {
                    count += 1;
                }
            }
            count
        }
    }
    print!("{} ", "".letters_count('a'));
    print!("{} ", "ddd".letters_count('a'));
    print!("{} ", "ddd".letters_count('d'));
    print!("{} ", "foobarbaz".letters_count('a'));
}
