#[macro_use]
extern crate criterion;
extern crate riiry;

//use criterion::Criterion;
use criterion::*;

use riiry::apps;
use riiry::files;
use riiry::filter;

fn bench_get_apps(c: &mut Criterion) {
    c.bench_function("bench_get_apps()", |b| {
        b.iter_batched(|| (), |_| apps::get_apps(), BatchSize::NumIterations(1))
    });
}

fn bench_get_files(c: &mut Criterion) {
    c.bench_function("bench_get_files()", |b| {
        b.iter_batched(|| (), |_| files::get_home_files(), BatchSize::NumIterations(1))
    });
}

fn bench_filter_lines_apps(c: &mut Criterion) {
    let apps = apps::get_apps().unwrap();

    c.bench_function("bench_filter_lines_apps()", move |b| {
        b.iter_batched(|| apps.clone(), |apps| {
            filter::filter_lines("firefox", &apps)
        }, BatchSize::NumIterations(1))
    });
}

fn bench_filter_lines_files(c: &mut Criterion) {
    let files = files::get_home_files().unwrap();
    c.bench_function("bench_filter_lines_files()", move |b| {
        b.iter_batched(|| files.clone(), |files| {
            filter::filter_lines("firefox", &files)
        }, BatchSize::NumIterations(1))
    });
}



criterion_group!(benches,
                 bench_get_apps,
                 bench_get_files,
                 bench_filter_lines_apps,
                 bench_filter_lines_files
                 );
criterion_main!(benches);
