// It does nothing.
fn main() {
    // OK: std::ops::Range<u32> is an iterator
    let _v1 = (0u32..10).next();

    // OK: std::ops::RangeFrom<u32> is an iterator
    let _v2 = (5u32..).next();

    // Illegal: std::ops::RangeTo<u32> is not an iterator
    // let _v3 = (..8u32).next();

    // Illegal: std::ops::RangeFull is not an iterator
    // let _v4 = (..).next();
}
