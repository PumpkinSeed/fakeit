 #[macro_use]
extern crate bencher;
use crate::beer;

fn bench_beer_name(bench: &mut Bencher) {
    bench.iter(|| beer::name());
}

benchmark_group!(benches, bench_beer_name);
benchmark_main!(benches);