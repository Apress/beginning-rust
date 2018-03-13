// It does nothing.
fn main() {
    enum Continent {
        Europe,
        Asia,
        Africa,
        America,
        Oceania,
    }

    let mut contin = Continent::Asia;

    match contin {
        Continent::Europe => {
            contin = Continent::Asia;
            print!("E");
        },
        Continent::Asia => { let a = 7; },
        Continent::Africa => print!("Af"),
        Continent::America => print!("Am"),
        Continent::Oceania => print!("O"),
    }
}
