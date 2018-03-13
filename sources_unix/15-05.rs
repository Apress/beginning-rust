// ILLEGAL
fn main() {
    let r1 = 3u8..12i8;
    let r2: std::ops::Range<u32> = -3..12;
    let r3: std::ops::Range<i32> = 3i16..12;
}
