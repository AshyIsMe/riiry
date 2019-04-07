#[macro_use]
extern crate criterion;
extern crate riiry;

use criterion::Criterion;

use riiry::apps;
use riiry::files;
use riiry::filter;

fn bench_get_apps(c: &mut Criterion) {
    c.bench_function("bench_get_apps()", |b| {
        b.iter(|| apps::get_apps())
    });
}

fn bench_get_files(c: &mut Criterion) {
    c.bench_function("bench_get_files()", |b| {
        b.iter(|| files::get_home_files())
    });
}

fn bench_filter_lines_apps(c: &mut Criterion) {
    c.bench_function("bench_filter_lines_apps()", |b| {
        b.iter(|| {
            let apps = apps::get_apps().unwrap();
            filter::filter_lines("firefox", &apps)
        })
    });
}

fn bench_filter_lines_files(c: &mut Criterion) {
    c.bench_function("bench_filter_lines_files()", |b| {
        b.iter(|| {
            let files = files::get_home_files().unwrap();
            filter::filter_lines("firefox", &files)
        })
    });
}



criterion_group!(benches,
                 bench_get_apps,
                 bench_get_files,
                 bench_filter_lines_apps,
                 bench_filter_lines_files
                 );
criterion_main!(benches);
