//! Performance benchmarks for kosync

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use kosync::{
    db::DB,
    defs::ProgressState,
    utils::{is_valid_field, is_valid_key_field, now_timestamp},
};
use std::path::PathBuf;
use tempfile::tempdir;

// Import test utilities
use crate::common::fixtures::{default_test_progress_state, default_test_user};

pub fn utils_benchmark(c: &mut Criterion) {
    // Benchmark is_valid_field
    c.bench_function("is_valid_field", |b| {
        b.iter(|| is_valid_field(black_box("test-field")))
    });

    // Benchmark is_valid_key_field
    c.bench_function("is_valid_key_field", |b| {
        b.iter(|| is_valid_key_field(black_box("test-key-field")))
    });

    // Benchmark now_timestamp
    c.bench_function("now_timestamp", |b| b.iter(|| now_timestamp()));
}

pub fn db_benchmark(c: &mut Criterion) {
    // Create a temporary directory for the test database
    let temp_dir = tempdir().expect("Failed to create temp directory");
    let db_path = temp_dir.path().join("bench_db");

    // Create a new database
    let db = DB::new(&db_path).expect("Failed to create database");

    // Create test data
    let (username, password) = default_test_user();
    let progress_state = default_test_progress_state();
    let document = progress_state.document.clone();

    // Put a user for benchmarking
    db.put_user(&username, &password)
        .expect("Failed to put user");

    // Benchmark get_user
    c.bench_function("db_get_user", |b| {
        b.iter(|| {
            let _ = db.get_user(black_box(&username));
        })
    });

    // Benchmark put_doc
    c.bench_function("db_put_doc", |b| {
        b.iter(|| {
            let _ = db.put_doc(
                black_box(&username),
                black_box(&document),
                black_box(&progress_state),
            );
        })
    });

    // Benchmark get_doc
    c.bench_function("db_get_doc", |b| {
        b.iter(|| {
            let _ = db.get_doc(black_box(&username), black_box(&document));
        })
    });
}

// Configure criterion groups
criterion_group!(benches, utils_benchmark, db_benchmark);
criterion_main!(benches);

// This function is required to make the benchmarks work with the common module
#[test]
fn it_works() {
    assert!(true);
}