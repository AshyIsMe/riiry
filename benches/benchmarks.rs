extern crate criterion;
extern crate riiry;

//use criterion::Criterion;
use criterion::*;
use std::path::{PathBuf};

use riiry::apps;
use riiry::files;
use riiry::filter;

fn pathbufs_to_vecstr(pathbufs: Vec<PathBuf>) -> Vec<String> {
    pathbufs
    .into_iter()
    .map(|pathbuf| {
        //pathbuf.to_str().map_or("", |s| format!("{}\n", s))
        pathbuf.to_str().unwrap_or_default().to_string()
    })
    .collect()
}

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
    let haystack = pathbufs_to_vecstr(apps);

    c.bench_function("bench_filter_lines_apps()", move |b| {
        b.iter_batched(|| haystack.clone(), |apps| {
            filter::filter_lines("firefox", apps)
        }, BatchSize::NumIterations(1))
    });
}

fn bench_filter_lines_apps_rff(c: &mut Criterion) {
    let apps = apps::get_apps().unwrap();
    let haystack = pathbufs_to_vecstr(apps);

    c.bench_function("bench_filter_lines_apps_rff()", move |b| {
        b.iter_batched(|| haystack.clone(), |apps| {
            filter::filter_lines_rff("firefox", &apps)
        }, BatchSize::NumIterations(1))
    });
}

fn bench_filter_lines_files(c: &mut Criterion) {
    let files = files::get_home_files().unwrap();
    let haystack = pathbufs_to_vecstr(files);

    c.bench_function("bench_filter_lines_files()", move |b| {
        b.iter_batched(|| haystack.clone(), |files| {
            filter::filter_lines("firefox", files)
        }, BatchSize::NumIterations(1))
    });
}



criterion_group!(benches,
                 //bench_get_apps,
                 //bench_get_files,
                 bench_filter_lines_apps,
                 bench_filter_lines_apps_rff,
                 //bench_filter_lines_files
                 );
criterion_main!(benches);
