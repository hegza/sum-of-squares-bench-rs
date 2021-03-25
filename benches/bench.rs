use criterion::{
    black_box, criterion_group, criterion_main, measurement::Measurement, BatchSize, Benchmark,
    BenchmarkGroup, BenchmarkId, Criterion, PlotConfiguration,
};
use float_ord::FloatOrd;
use rand::Rng;
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
const END_POW: u32 = 26;
const STEP_POW: u32 = 2;

// Top level measurement organizers

fn bench_data_structures(c: &mut Criterion) {
    compare_data_structures(START_POW, END_POW, STEP_POW, c);
}

fn bench_all_by_size(c: &mut Criterion) {
    measure_all_containers_by_sizes(START_POW, END_POW, STEP_POW, c);
}

fn bench_by_container(c: &mut Criterion) {
    measure_all_sizes_by_container(START_POW, END_POW, STEP_POW, c);
}

/// Try the biggest input to check how long the measurement takes
fn calibrate(c: &mut Criterion) {
    let mut group = c.benchmark_group("Calibrate");

    let input_size_bytes = 2u32.pow(END_POW) as usize;
    let data_len = input_size_bytes / std::mem::size_of::<f64>(); // 10 MB
    let input: Vec<FloatOrd<f64>> = create_scrambled_data(data_len);

    measure_sum_of_squares("Vec", input.clone(), &mut group);
}

// Level 2u32 measurement organizers

fn measure_all_containers_by_sizes(
    start_pow2: u32,
    end_pow2: u32,
    step_pow2: u32,
    c: &mut Criterion,
) {
    let mut input_size_bytes = 2u32.pow(start_pow2) as usize;
    while input_size_bytes <= 2u32.pow(end_pow2) as usize {
        let name = human_readable_size(input_size_bytes);
        let mut group = c.benchmark_group(name);

        // A 64-bit float is 8 bytes long, so we divide 1024 by 8 bytes to obtain the right data length
        let data_len = input_size_bytes / std::mem::size_of::<f64>();

        let input: Vec<FloatOrd<f64>> = create_scrambled_data(data_len);
        measure_sum_of_squares("Vec", input, &mut group);

        let input: VecDeque<FloatOrd<f64>> = create_scrambled_data(data_len);
        measure_sum_of_squares("VecDeque", input, &mut group);

        let input: LinkedList<FloatOrd<f64>> = create_scrambled_data(data_len);
        measure_sum_of_squares("LinkedList", input, &mut group);

        let input: HashSet<FloatOrd<f64>> = create_scrambled_data(data_len);
        measure_sum_of_squares("HashSet", input, &mut group);

        let input: BTreeSet<FloatOrd<f64>> = create_scrambled_data(data_len);
        measure_sum_of_squares("BTreeSet", input, &mut group);

        group.finish();
        input_size_bytes *= 2u32.pow(step_pow2) as usize;
    }
}

fn measure_all_sizes_by_container(
    start_pow2: u32,
    end_pow2: u32,
    step_pow2: u32,
    c: &mut Criterion,
) {
    measure_vec(start_pow2, end_pow2, step_pow2, c);
    measure_vec_deque(start_pow2, end_pow2, step_pow2, c);
    measure_linked_list(start_pow2, end_pow2, step_pow2, c);
    measure_hash_set(start_pow2, end_pow2, step_pow2, c);
    measure_btree_set(start_pow2, end_pow2, step_pow2, c);
}

fn measure_vec(start_pow2: u32, end_pow2: u32, step_pow2: u32, c: &mut Criterion) {
    measure_all_sizes::<Vec<FloatOrd<f64>>>("Vec", start_pow2, end_pow2, step_pow2, c);
}

fn measure_vec_deque(start_pow2: u32, end_pow2: u32, step_pow2: u32, c: &mut Criterion) {
    measure_all_sizes::<VecDeque<FloatOrd<f64>>>("VecDeque", start_pow2, end_pow2, step_pow2, c);
}

fn measure_linked_list(start_pow2: u32, end_pow2: u32, step_pow2: u32, c: &mut Criterion) {
    measure_all_sizes::<LinkedList<FloatOrd<f64>>>(
        "LinkedList",
        start_pow2,
        end_pow2,
        step_pow2,
        c,
    );
}

fn measure_hash_set(start_pow2: u32, end_pow2: u32, step_pow2: u32, c: &mut Criterion) {
    measure_all_sizes::<HashSet<FloatOrd<f64>>>("HashSet", start_pow2, end_pow2, step_pow2, c);
}

fn measure_btree_set(start_pow2: u32, end_pow2: u32, step_pow2: u32, c: &mut Criterion) {
    measure_all_sizes::<BTreeSet<FloatOrd<f64>>>("BTreeSet", start_pow2, end_pow2, step_pow2, c);
}

fn measure_all_sizes<T>(
    name: &str,
    start_pow2: u32,
    end_pow2: u32,
    step_pow2: u32,
    c: &mut Criterion,
) where
    T: Clone + iter::FromIterator<FloatOrd<f64>> + iter::IntoIterator<Item = FloatOrd<f64>>,
{
    let mut group = c.benchmark_group(name);

    let mut input_size_bytes = 2u32.pow(start_pow2) as usize;
    while input_size_bytes <= 2u32.pow(end_pow2) as usize {
        let name = human_readable_size(input_size_bytes);

        // A 64-bit float is 8 bytes long, so we divide 1024 by 8 bytes to obtain the right data length
        let data_len = input_size_bytes / std::mem::size_of::<f64>();

        let input: T = create_scrambled_data(data_len);

        measure_sum_of_squares(&name, input, &mut group);
        input_size_bytes *= 2u32.pow(step_pow2) as usize;
    }

    group.finish();
}

// Measurement structure
fn measure_sum_of_squares<T, M>(name: &str, input: T, group: &mut BenchmarkGroup<M>)
where
    T: Clone + iter::IntoIterator<Item = FloatOrd<f64>>,
    M: Measurement,
{
    group.bench_function(name, move |b| {
        // Per-sample (note that a sample can be many iterations) setup goes here
        b.iter_batched(
            || input.clone(),
            |data| {
                // Measured code goes here
                sum_of_squares(black_box(data))
            },
            BatchSize::LargeInput,
        )
    });
}

fn bench_in_group_with_input<T, M>(
    ds_name: &str,
    parameter_name: &str,
    data_len: usize,
    group: &mut BenchmarkGroup<M>,
) where
    T: Clone + iter::FromIterator<FloatOrd<f64>> + iter::IntoIterator<Item = FloatOrd<f64>>,
    M: Measurement,
{
    group.bench_with_input(
        BenchmarkId::new(ds_name, parameter_name),
        &data_len,
        |b, data_len| {
            b.iter_batched(
                || create_scrambled_data::<T>(*data_len),
                |data| sum_of_squares(black_box(data)),
                BatchSize::LargeInput,
            )
        },
    );
}

fn compare_data_structures(start_pow2: u32, end_pow2: u32, step_pow2: u32, c: &mut Criterion) {
    let mut group = c.benchmark_group("Sum of squares");

    let conf = PlotConfiguration::default().summary_scale(criterion::AxisScale::Logarithmic);
    group.plot_config(conf);
    group.sampling_mode(criterion::SamplingMode::Linear);

    let mut input_size_bytes = 2u32.pow(start_pow2) as usize;
    while input_size_bytes <= 2u32.pow(end_pow2) as usize {
        // Give input length in bytes to configure criterion
        group.throughput(criterion::Throughput::Bytes(input_size_bytes as u64));

        // A 64-bit float is 8 bytes long, so we divide 1024 by 8 bytes to obtain the right data length
        let data_len = input_size_bytes / std::mem::size_of::<f64>();
        let par_name = human_readable_size(input_size_bytes);

        bench_in_group_with_input::<Vec<FloatOrd<f64>>, _>("Vec", &par_name, data_len, &mut group);
        bench_in_group_with_input::<VecDeque<FloatOrd<f64>>, _>(
            "VecDeque", &par_name, data_len, &mut group,
        );
        bench_in_group_with_input::<LinkedList<FloatOrd<f64>>, _>(
            "LinkedList",
            &par_name,
            data_len,
            &mut group,
        );
        bench_in_group_with_input::<HashSet<FloatOrd<f64>>, _>(
            "HashSet", &par_name, data_len, &mut group,
        );
        bench_in_group_with_input::<BTreeSet<FloatOrd<f64>>, _>(
            "BTreeSet", &par_name, data_len, &mut group,
        );

        input_size_bytes *= 2u32.pow(step_pow2) as usize;
    }

    group.finish();
}

// Init data

fn create_scrambled_data<T>(n: usize) -> T
where
    T: FromIterator<FloatOrd<f64>>,
{
    let mut rng = rand::thread_rng();

    (0..n).into_iter().map(|_| FloatOrd(rng.gen())).collect()
}

// Final data loop used by everything

fn sum_of_squares<T>(collection: T) -> f64
where
    T: iter::IntoIterator<Item = FloatOrd<f64>>,
{
    spp_experiments::sum_of_squares(collection)
}

// Criterion setup

criterion_group!(benches, bench_data_structures);
criterion_main!(benches);
