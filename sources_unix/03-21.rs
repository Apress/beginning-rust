/* It prints:
true false*/
fn main() {
    print!("{} {}", true || true && ! true,
        (true || true) && ! true);
}
