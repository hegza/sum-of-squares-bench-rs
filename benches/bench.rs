use criterion::{
    black_box, criterion_group, criterion_main, measurement::Measurement, BatchSize,
    BenchmarkGroup, BenchmarkId, Criterion, PlotConfiguration,
};
use float_ord::FloatOrd;
use rand::Rng;
use spp_experiments::Float;
use std::collections::{BTreeSet, HashSet, LinkedList, VecDeque};
use std::iter::{self, FromIterator};

fn human_readable_size(size_bytes: usize) -> String {
    if size_bytes < 1024 {
        size_bytes.to_string() + " bytes"
    } else if size_bytes < 1024 * 1024 {
        (size_bytes / 1024).to_string() + " kB"
    } else if size_bytes < 1024 * 1024 * 1024 {
        (size_bytes / 1024 / 1024).to_string() + " MB"
    } else if size_bytes < 1024 * 1024 * 1024 {
        (size_bytes / 1024 / 1024).to_string() + " GB"
    } else {
        size_bytes.to_string() + " ??"
    }
}

// Powers of 2u32 limits for measurements
// 10 = 1 kB, 20 = 1 MB
// 24 = 16 MB = L3 cache size on test platform
// 26 = 64 MB = target
// 30 = 1 GB
const START_POW: u32 = 10;
const END_POW: u32 = 26; // 26 for final measurements
const STEP_POW: u32 = 2;

// Top level measurement organizers

fn bench_data_structures(c: &mut Criterion) {
    compare_data_structures(START_POW, END_POW, STEP_POW, c);
}

fn compare_data_structures(start_pow2: u32, end_pow2: u32, step_pow2: u32, c: &mut Criterion) {
    let mut group = c.benchmark_group("Sum of squares");

    let conf = PlotConfiguration::default().summary_scale(criterion::AxisScale::Logarithmic);
    group.plot_config(conf);
    // Force linear sampling mode for everything, the 1 MB+ samples will be a bit slow but that's fine
    group.sampling_mode(criterion::SamplingMode::Linear);

    // Iterate over data-sizes of powers of two from START_POW to END_POW
    let mut input_size_bytes = 2u32.pow(start_pow2) as usize;
    while input_size_bytes <= 2u32.pow(end_pow2) as usize {
        // Give input length in bytes to configure criterion
        group.throughput(criterion::Throughput::Bytes(input_size_bytes as u64));

        // A 64-bit float is 8 bytes long, so we divide 1024 by 8 bytes to obtain the
        // right data length
        let data_len = input_size_bytes / std::mem::size_of::<f64>();
        let input_bytes_human = human_readable_size(input_size_bytes);

        // Run all the benchmarks with this input size
        bench_data_structures_in_group_with_input::<FloatOrd<f64>, _>(
            &input_bytes_human,
            data_len,
            &mut group,
        );

        input_size_bytes *= 2u32.pow(step_pow2) as usize;
    }

    group.finish();
}

fn bench_data_structures_in_group_with_input<V, M>(
    input_bytes_human: &str,
    data_len: usize,
    group: &mut BenchmarkGroup<M>,
) where
    V: Float<f64>,
    M: Measurement,
{
    bench_by_ref_in_group::<V, Vec<V>, _>(
        "Vec (by reference)",
        &input_bytes_human,
        data_len,
        group,
    );
    bench_by_ref_in_group::<V, VecDeque<V>, _>(
        "VecDeque (by reference)",
        &input_bytes_human,
        data_len,
        group,
    );
    bench_by_ref_in_group::<V, LinkedList<V>, _>(
        "LinkedList (by reference)",
        &input_bytes_human,
        data_len,
        group,
    );
    bench_by_ref_in_group::<V, HashSet<V>, _>(
        "HashSet (by reference)",
        &input_bytes_human,
        data_len,
        group,
    );
    bench_by_ref_in_group::<V, BTreeSet<V>, _>(
        "BTreeSet (by reference)",
        &input_bytes_human,
        data_len,
        group,
    );

    bench_by_val_in_group::<V, Vec<V>, _>("Vec (by value)", &input_bytes_human, data_len, group);
    bench_by_val_in_group::<V, VecDeque<V>, _>(
        "VecDeque (by value)",
        &input_bytes_human,
        data_len,
        group,
    );
    bench_by_val_in_group::<V, LinkedList<V>, _>(
        "LinkedList (by value)",
        &input_bytes_human,
        data_len,
        group,
    );
    bench_by_val_in_group::<V, HashSet<V>, _>(
        "HashSet (by value)",
        &input_bytes_human,
        data_len,
        group,
    );
    bench_by_val_in_group::<V, BTreeSet<V>, _>(
        "BTreeSet (by value)",
        &input_bytes_human,
        data_len,
        group,
    );
}

fn bench_by_ref_in_group<V, T, M>(
    ds_name: &str,
    parameter_name: &str,
    data_len: usize,
    group: &mut BenchmarkGroup<M>,
) where
    V: Float<f64>,
    T: iter::FromIterator<V> + iter::IntoIterator<Item = V> + Clone,
    for<'a> &'a T: iter::IntoIterator<Item = &'a V>,
    M: Measurement,
{
    // Create concrete data-structure using FromIterator<V>
    let data: T = create_scrambled_data(data_len);

    group.bench_function(BenchmarkId::new(ds_name, parameter_name), move |b| {
        b.iter_batched(
            || data.clone(),
            |data| sum_of_squares_by_ref(black_box(&data)),
            BatchSize::LargeInput,
        )
    });
}

fn bench_by_val_in_group<V, T, M>(
    ds_name: &str,
    parameter_name: &str,
    data_len: usize,
    group: &mut BenchmarkGroup<M>,
) where
    V: Float<f64>,
    T: iter::FromIterator<V> + iter::IntoIterator<Item = V> + Clone + iter::IntoIterator<Item = V>,
    M: Measurement,
{
    // Create concrete data-structure using FromIterator<V>
    let data: T = create_scrambled_data(data_len);

    group.bench_function(BenchmarkId::new(ds_name, parameter_name), move |b| {
        b.iter_batched(
            || data.clone(),
            |data| sum_of_squares_by_move(black_box(data)),
            BatchSize::LargeInput,
        )
    });
}

/// Create the concrete data-structure of length `n` using FromIterator<V> where V is the element type.
fn create_scrambled_data<V, T>(n: usize) -> T
where
    V: Float<f64>,
    T: FromIterator<V>,
{
    let mut rng = rand::thread_rng();

    (0..n).into_iter().map(|_| V::create(rng.gen())).collect()
}

// Final data loop used by everything

fn sum_of_squares_by_ref<V, T>(collection: &T) -> f64
where
    V: Float<f64>,
    for<'a> &'a T: iter::IntoIterator<Item = &'a V>,
{
    spp_experiments::sum_of_squares_by_ref(collection)
}

fn sum_of_squares_by_move<V, T>(collection: T) -> f64
where
    V: Float<f64>,
    T: iter::IntoIterator<Item = V>,
{
    spp_experiments::sum_of_squares_by_move(collection)
}

// Criterion setup

criterion_group!(benches, bench_data_structures);
criterion_main!(benches);
