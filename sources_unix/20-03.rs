/* After some time of computation,
it prints three fractional numbers.*/
fn main() {
    use std::time::Instant;
    fn elapsed_ms(t1: Instant, t2: Instant) -> f64 {
        let t = t2 - t1;
        t.as_secs() as f64 * 1000.
            + t.subsec_nanos() as f64 / 1e6
    }

    const SIZE: usize = 100_000;
    let t0 = Instant::now();
    let mut v = Vec::<usize>::with_capacity(SIZE);
    let t1 = Instant::now();
    for i in 0..SIZE {
        v.insert(0, i);
    }
    let t2 = Instant::now();
    for _ in 0..SIZE {
        v.remove(0);
    }
    let t3 = Instant::now();
    print!("{} {} {}", elapsed_ms(t0, t1),
        elapsed_ms(t1, t2), elapsed_ms(t2, t3));
}
