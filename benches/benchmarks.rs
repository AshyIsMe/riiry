#[macro_use]
extern crate criterion;
extern crate riiry;

use criterion::Criterion;

use riiry::filter;

fn bench_filter_lines(c: &mut Criterion) {
    c.bench_function("filter_lines()", |b| b.iter(|| filter::filter_lines("foo", "foo\nbar\nbaz")));
}

criterion_group!(benches, bench_filter_lines);
criterion_main!(benches);
