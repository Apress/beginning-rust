/* After some time of computation, it prints:
Pushes in Vec: <fractional number>
Insertions in HashSet: <fractional number>
Insertions in BTreeSet: <fractional number>
Linear search in Vec: <fractional number>
Sort of Vec: <fractional number>
Binary search in Vec: <fractional number>
Search in HashSet: <fractional number>
Search in BTreeSet: <fractional number>
*/
fn main() {
    use std::time::Instant;
    fn elapsed_ms(t1: Instant, t2: Instant) -> f64 {
        let t = t2 - t1;
        t.as_secs() as f64 * 1000.
            + t.subsec_nanos() as f64 / 1e6
    }

    const SIZE: i32 = 40_000;
    fn ns_per_op(t1: Instant, t2: Instant) -> f64 {
        elapsed_ms(t1, t2) / SIZE as f64 * 1_000_000.
    }
    let mut v = Vec::<_>::new();
    let mut hs = std::collections::HashSet::<_>::new();
    let mut bs = std::collections::BTreeSet::<_>::new();
    let t0 = Instant::now();
    for i in 0..SIZE { v.push(i); }
    let t1 = Instant::now();
    for i in 0..SIZE { hs.insert(i); }
    let t2 = Instant::now();
    for i in 0..SIZE { bs.insert(i); }
    let t3 = Instant::now();
    for i in 0..SIZE { if ! v.contains(&i) { return; } }
    let t4 = Instant::now();
    v.swap(10_000, 20_000);
    v.sort();
    let t5 = Instant::now();
    for i in 0..SIZE {
        if v.binary_search(&i).is_err() { return; }
    }
    let t6 = Instant::now();
    for i in 0..SIZE { if ! hs.contains(&i) { return; } }
    let t7 = Instant::now();
    for i in 0..SIZE { if ! bs.contains(&i) { return; } }
    let t8 = Instant::now();
    println!("Pushes in Vec: {}", ns_per_op(t0, t1));
    println!("Insertions in HashSet: {}", ns_per_op(t1, t2));
    println!("Insertions in BTreeSet: {}", ns_per_op(t2, t3));
    println!("Linear search in Vec: {}", ns_per_op(t3, t4));
    println!("Sort of Vec: {}", ns_per_op(t4, t5));
    println!("Binary search in Vec: {}", ns_per_op(t5, t6));
    println!("Search in HashSet: {}", ns_per_op(t6, t7));
    println!("Search in BTreeSet: {}", ns_per_op(t7, t8));
}
