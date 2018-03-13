/* It waits for console input.
   If the user types a "b" and then hits the Enter key,
   the program prints:
[Hi]
   If the user types anything else or nothing,
   and then hits the Enter key, the program prints:
Hello*/
fn main() {
    trait Draw {
        fn draw(&self);
    }
    
    struct Text { characters: String }
    impl Text {
        fn from(text: &str) -> Text {
            Text { characters: text.to_string() }
        }
    }
    impl Draw for Text {
        fn draw(&self) {
            print!("{}", self.characters);
        }
    }

    struct BoxedText {
        text: Text,
        first: char,
        last: char,
    }
    impl BoxedText {
        fn with_text_and_borders(
            text: &str, first: char, last: char)
            -> BoxedText
        {
            BoxedText {
                text: Text::from(text),
                first: first,
                last: last,
            }
        }
    }
    impl Draw for BoxedText {
        fn draw(&self) {
            print!("{}", self.first);
            self.text.draw();
            print!("{}", self.last);
        }
    }

    let greeting = Text::from("Hello");
    let boxed_greeting =
        BoxedText::with_text_and_borders("Hi", '[', ']');

    // SOLUTION 2/bis //
    fn draw_text(txt: &Draw) {
        txt.draw();
    }
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    let dr: &Draw = if input.trim() == "b" {
        &boxed_greeting
    } else {
        &greeting
    };
    draw_text(dr);
}
