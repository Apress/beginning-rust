// ILLEGAL
fn main() {
    let arr = [11, 22, 33];
    let i: usize = 2;
    print!("{}", arr[i]);
    let i: isize = 2;
    print!("{}", arr[i]);
    let i: u32 = 2;
    print!("{}", arr[i]);
    let i: u64 = 2;
    print!("{}", arr[i]);
}
