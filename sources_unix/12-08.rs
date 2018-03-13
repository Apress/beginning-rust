/* It prints:
<an integer number> <an integer number> <an integer number>*/
fn main() {
    let b1 = true;
    let b2 = true;
    let b3 = false;
    print!("{} {} {}",
        &b1 as *const bool as usize,
        &b2 as *const bool as usize,
        &b3 as *const bool as usize);
}
