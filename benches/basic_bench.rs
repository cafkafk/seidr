// SPDX-FileCopyrightText: 2023 Christina Sørensen
// SPDX-FileContributor: Christina Sørensen
//
// SPDX-License-Identifier: AGPL-3.0-only

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use relative_path::RelativePath;
use seidr::git::Config;

pub fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("config loading time", |b| {
        b.iter(|| {
            Config::new(black_box(
                &RelativePath::new(black_box("./src/test/config.yaml")).to_string(),
            ))
        });
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
