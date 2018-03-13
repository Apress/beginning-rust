/* After some time of computation,
it prints two fractional numbers.*/
fn main() {
    use std::time::Instant;
    fn elapsed_ms(t1: Instant, t2: Instant) -> f64 {
        let t = t2 - t1;
        t.as_secs() as f64 * 1000.
            + t.subsec_nanos() as f64 / 1e6
    }

    const SIZE: usize = 40_000;
    let t0 = Instant::now();
    let mut v = Vec::<usize>::new();
    for i in 0..SIZE {
        v.push(i);
        v.push(SIZE + i);
        v.remove(0);
        v.push(SIZE * 2 + i);
        v.remove(0);
    }
    let t1 = Instant::now();
    while v.len() > 0 {
        v.remove(0);
    }
    let t2 = Instant::now();
    print!("{} {}", elapsed_ms(t0, t1), elapsed_ms(t1, t2));
}
