/* After some seconds of computation, it prints:
<fractional number> ns per iteration
*/
fn main() {
    use std::time::Instant;
    fn elapsed_ms(t1: Instant, t2: Instant) -> f64 {
        let t = t2 - t1;
        t.as_secs() as f64 * 1000. + t.subsec_nanos() as f64 / 1e6
    }
    const N_ITER: usize = 100_000_000;
    let start_time = Instant::now();
    for i in 0..N_ITER {
        let v1 = vec![11, 22];
        let mut v2 = v1; // Move semantics is used
        v2.push(i);
        if v2[1] + v2[2] == v2[0] {
            print!("Error");
        }
    }
    let finish_time = Instant::now();
    print!("{} ns per iteration\n",
        elapsed_ms(start_time, finish_time) * 1e6 / N_ITER as f64);
}
