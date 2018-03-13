/* After some time of computation,
it prints five fractional numbers.*/
fn main() {
    use std::time::Instant;
    fn elapsed_ms(t1: Instant, t2: Instant) -> f64 {
        let t = t2 - t1;
        t.as_secs() as f64 * 1000.
            + t.subsec_nanos() as f64 / 1e6
    }

    const SIZE: usize = 40_000;
    let mut v = Vec::<usize>::new();
    let mut vd = std::collections::VecDeque::<usize>::new();
    let t0 = Instant::now();
    for i in 0..SIZE {
        v.push(i);
    }
    let t1 = Instant::now();
    for i in 0..SIZE {
        vd.push_back(i);
    }
    let mut count = 0;
    let t2 = Instant::now();
    for i in v.iter() {
        count += i;
    }
    let t3 = Instant::now();
    for i in vd.iter() {
        count += i;
    }
    let t4 = Instant::now();
    print!("{} {} {} {} {}", count,
        elapsed_ms(t0, t1), elapsed_ms(t1, t2),
        elapsed_ms(t2, t3), elapsed_ms(t3, t4));
}
