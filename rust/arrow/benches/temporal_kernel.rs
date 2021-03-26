// Licensed to the Apache Software Foundation (ASF) under one
// or more contributor license agreements.  See the NOTICE file
// distributed with this work for additional information
// regarding copyright ownership.  The ASF licenses this file
// to you under the Apache License, Version 2.0 (the
// "License"); you may not use this file except in compliance
// with the License.  You may obtain a copy of the License at
//
//   http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing,
// software distributed under the License is distributed on an
// "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY
// KIND, either express or implied.  See the License for the
// specific language governing permissions and limitations
// under the License.

extern crate arrow;
#[macro_use]
extern crate criterion;

use criterion::Criterion;

use arrow::array::{PrimitiveArray, PrimitiveBuilder};
use arrow::compute::kernels::temporal::hour;
use arrow::compute::year;
use arrow::datatypes::Date64Type;
use arrow::util::test_util::seedable_rng;
use rand::distributions::Uniform;
use rand::Rng;

fn build_date64_array(size: usize, null_fraction: f32) -> PrimitiveArray<Date64Type> {
    let mut rng = seedable_rng();
    let mut builder: PrimitiveBuilder<Date64Type> = PrimitiveArray::builder(size);
    let range = Uniform::new(0, 1608071414123);

    for _ in 0..size {
        if null_fraction > 0.0 && rng.gen::<f32>() < null_fraction {
            builder.append_null().unwrap();
        } else {
            builder.append_value(rng.sample(range)).unwrap();
        }
    }
    builder.finish()
}

fn add_benchmark(c: &mut Criterion) {
    let nulls: PrimitiveArray<Date64Type> = build_date64_array(100, 1.0);
    let mixed: PrimitiveArray<Date64Type> = build_date64_array(100, 0.2);
    let solid: PrimitiveArray<Date64Type> = build_date64_array(100, 0.0);

    let mut group = c.benchmark_group("temporal/hour");
    group.bench_function("nulls", |b| b.iter(|| criterion::black_box(hour(&nulls))));
    group.bench_function("mixed", |b| b.iter(|| criterion::black_box(hour(&mixed))));
    group.bench_function("solid", |b| b.iter(|| criterion::black_box(hour(&solid))));
    group.finish();

    let mut group = c.benchmark_group("temporal/year");
    group.bench_function("nulls", |b| b.iter(|| criterion::black_box(year(&nulls))));
    group.bench_function("mixed", |b| b.iter(|| criterion::black_box(year(&mixed))));
    group.bench_function("solid", |b| b.iter(|| criterion::black_box(year(&solid))));
    group.finish();
}

criterion_group!(benches, add_benchmark);
criterion_main!(benches);
